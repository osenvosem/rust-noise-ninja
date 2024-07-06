use crate::components::{control_panel::ControlPanel, grid::Grid, sound_library::SoundLibrary};
use crate::shared::{
    generate_lib, Category, Operation, Sample, DEFAULT_GRID_SIZE, EMPTY_SOUND, GRID_COLUMN_STEP,
    SOUND_LIB_PATH,
};
use html::Audio;
use leptos::*;
use leptos_dom::helpers::TimeoutHandle;
use rand::{thread_rng, Rng};
use std::time::Duration;
use wasm_bindgen::closure::Closure;

#[component]
pub fn App() -> impl IntoView {
    let mut grid_data_initial = vec![None; usize::from(DEFAULT_GRID_SIZE * 6)];
    fill_grid_initial(&mut grid_data_initial);
    let container_class = "";

    let (grid_data, set_grid_data) = create_signal::<Vec<Option<Sample>>>(grid_data_initial);
    let (play, set_play) = create_signal(false);
    let (duration, set_duration) = create_signal(0);
    let (current_cell, set_current_cell) = create_signal(0);
    let (timeout_handler, set_timeout_handler) = create_signal::<Option<TimeoutHandle>>(None);
    let (volume, set_volume) = create_signal::<f32>(1.0);
    let (edit_cell_idx, set_edit_cell_idx) = create_signal::<Option<u16>>(None);
    let (random_playback, set_random_playback) = create_signal(false);
    let sound_lib = create_resource(|| {}, |_| async { generate_lib().await });

    let main_audio_elem_ref = create_node_ref::<Audio>();
    let secondary_audio_elem_ref = create_node_ref::<Audio>();

    let grid_size_handler = move |op: Operation| {
        let mut gd = grid_data.get();
        let len = gd.len();
        set_grid_data.set(match op {
            Operation::Dec => {
                gd.drain(len - GRID_COLUMN_STEP as usize..);
                gd
            }
            Operation::Inc => {
                gd.splice(len.., vec![None; GRID_COLUMN_STEP as usize]);
                gd
            }
        });
    };

    let ended_listener = move |_| {
        let handler = set_timeout_with_handle(
            move || {
                set_current_cell.update(move |val| {
                    let len = grid_data.get().len();
                    *val = if random_playback.get() {
                        thread_rng().gen_range(0..len)
                    } else if *val == len - 1 {
                        0
                    } else {
                        *val + 1
                    }
                })
            },
            Duration::from_millis(duration.get()),
        )
        .ok();

        set_timeout_handler.set(handler);
    };

    create_effect(move |_| {
        let audio = main_audio_elem_ref
            .get()
            .expect("Failed to get ref to audio element");

        if play.get() {
            if let Some(sample_opt) = grid_data.get().get(current_cell.get()) {
                if let Some(sample) = sample_opt {
                    audio.set_src(&sample.filepath);

                    if let Ok(promise) = audio.play() {
                        let reject_handler = Closure::new(move |err| {
                            logging::error!("{:?}", err);
                        });
                        let _ = promise.catch(&reject_handler);
                        reject_handler.forget();
                    }
                } else {
                    audio.set_src(EMPTY_SOUND);
                    if let Ok(promise) = audio.play() {
                        let reject_handler = Closure::new(move |err| {
                            logging::error!("{:?}", err);
                        });
                        let _ = promise.catch(&reject_handler);
                        reject_handler.forget();
                    }
                }
            } else {
                set_current_cell(0);
            }
        } else if let Some(timeout_handle) = timeout_handler.get() {
            timeout_handle.clear();
            set_timeout_handler.set(None);
            let _ = audio.pause();
            audio.set_current_time(0.0);
        }
    });

    let grid_cell_click_handler =
        Callback::new(move |(sound_url_opt, idx): (Option<String>, u16)| {
            let audio = secondary_audio_elem_ref
                .get()
                .expect("Failed to get ref to secondary audio element");

            if let Some(sound_url) = sound_url_opt {
                audio.set_src(&sound_url);
                let _ = audio.play();
            }

            set_current_cell.set(idx as usize);
        });

    let sample_select_handler = move |sample| {
        let idx = edit_cell_idx.get().unwrap();
        set_edit_cell_idx.set(None);
        let mut mut_grid_data = grid_data.get();
        mut_grid_data[idx as usize] = Some(sample);
        set_grid_data.set(mut_grid_data);
    };

    let open_library_handler = move |idx| {
        set_edit_cell_idx.set(Some(idx));
    };

    let close_library_handler = move |_| {
        set_edit_cell_idx.set(None);
    };

    let clear_cell_handler = move |_| {
        let idx = edit_cell_idx.get().unwrap();
        set_edit_cell_idx.set(None);
        let mut mut_grid_data = grid_data.get();
        mut_grid_data[idx as usize] = None;
        set_grid_data.set(mut_grid_data);
    };

    let is_cell_filled = Signal::derive(move || {
        if let Some(idx) = edit_cell_idx.get() {
            grid_data.get().get(idx as usize).unwrap().is_some()
        } else {
            false
        }
    });

    view! {
        <div class=container_class>
            <Grid
                grid_data
                current_cell
                click_handler=grid_cell_click_handler
                open_library_handler
            />
            <ControlPanel
                play
                set_play
                duration
                set_duration
                grid_rows_num=Signal::derive(move || {
                    grid_data.get().len() as u16 / DEFAULT_GRID_SIZE
                })

                grid_size_handler
                volume
                set_volume
                random_playback
                set_random_playback
            />
            <Suspense fallback=move || view! { "" }>
                <ErrorBoundary fallback=|_| {view! {<p>"Something went wrong"</p>}}>
                    {move || {
                        sound_lib.get().map(|lib| {
                            view! {
                                    <SoundLibrary
                                        sound_lib=lib
                                        edit_cell_idx
                                        volume
                                        is_cell_filled
                                        sample_select_handler
                                        close_library_handler
                                        clear_cell_handler
                                    />

                            }

                        })
                    }}
                </ErrorBoundary>
            </Suspense>
            <audio _ref=main_audio_elem_ref prop:volume=volume on:ended=ended_listener></audio>
            <audio _ref=secondary_audio_elem_ref prop:volume=volume></audio>
        </div>
    }
}

fn fill_grid_initial(grid_data_initial: &mut [Option<Sample>]) {
    let sample = Sample {
        id: "boom_ball_1".to_string(),
        filepath: format!("{SOUND_LIB_PATH}boom/ball_1.mp3"),
        category: Category::Boom,
        filename: "ball_1".to_string(),
        duration: 0.32567,
    };

    let mod_idx = [0, 2, 3, 12, 23, 34];

    for (idx, item) in grid_data_initial.iter_mut().enumerate() {
        if mod_idx.contains(&idx) {
            *item = Some(sample.clone());
        }
    }
}
