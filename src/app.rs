use crate::components::{
    control_panel::ControlPanel, grid::Grid, presets::Presets, sound_library::SoundLibrary,
};
use crate::shared::{
    Category, Operation, Sample, DEFAULT_GRID_SIZE, EMPTY_SOUND, GRID_COLUMN_STEP, GRID_ROWS_MAX,
    GRID_ROWS_MIN, SOUND_LIB_JSON_PATH, SOUND_LIB_PATH,
};
use html::Audio;
use leptos::*;
use leptos_dom::helpers::TimeoutHandle;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::time::Duration;
use wasm_bindgen::{closure::Closure, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
extern "C" {
    type Store;

    #[wasm_bindgen(constructor, js_namespace = __TAURI_PLUGIN_STORE__)]
    fn new(filename: &str) -> Store;

    #[wasm_bindgen(method)]
    async fn set(this: &Store, key: &str, value: &str) -> JsValue;

    #[wasm_bindgen(method)]
    async fn get(this: &Store, key: &str) -> JsValue;

    #[wasm_bindgen(method)]
    async fn delete(this: &Store, key: &str) -> JsValue;

    #[wasm_bindgen(method)]
    async fn save(this: &Store);

    #[wasm_bindgen(method)]
    async fn clear(this: &Store);

    #[wasm_bindgen(method)]
    async fn keys(this: &Store) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (grid_data, set_grid_data) = create_signal::<Vec<Option<Sample>>>(Vec::new());
    let (play, set_play) = create_signal(false);
    let (gap_duration, set_gap_duration) = create_signal(1000);
    let (current_cell, set_current_cell) = create_signal(0);
    let (timeout_handler, set_timeout_handler) = create_signal::<Option<TimeoutHandle>>(None);
    let (volume, set_volume) = create_signal::<f32>(1.0);
    let (edit_cell_idx, set_edit_cell_idx) = create_signal::<Option<u16>>(None);
    let (random_playback, set_random_playback) = create_signal(false);
    let (save_blocked, set_save_blocked) = create_signal(false);
    let (presets_visible, set_presets_visible) = create_signal(false);

    let sound_lib = create_resource(
        || {},
        |_| async {
            let resp_val = JsFuture::from(
                web_sys::window()
                    .unwrap()
                    .fetch_with_str(SOUND_LIB_JSON_PATH),
            )
            .await
            .unwrap();

            let resp: Response = resp_val.dyn_into().unwrap();
            let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

            // NOTE: Sort vectors in the hashmap once
            serde_wasm_bindgen::from_value::<HashMap<Category, Vec<Sample>>>(json)
                .unwrap()
                .iter_mut()
                .map(|(c, v)| {
                    v.sort();
                    (*c, v.clone())
                })
                .collect()
        },
    );

    let main_audio_elem_ref = create_node_ref::<Audio>();
    let secondary_audio_elem_ref = create_node_ref::<Audio>();

    // NOTE: Restore state
    create_effect(move |_| {
        set_save_blocked.set(true);

        let store = Store::new("foo.bin");

        wasm_bindgen_futures::spawn_local(async move {
            // store.clear().await;
            if let Ok(gap_duration_js_val) =
                serde_wasm_bindgen::from_value::<String>(store.get("duration").await)
            {
                if let Ok(gap_duration) = gap_duration_js_val.parse::<u64>() {
                    set_gap_duration.set(gap_duration);
                }
            }

            if let Ok(random_playback) = serde_wasm_bindgen::from_value::<String>(
                store.get("random_playback").await,
            )
            .map(|val| match val.as_str() {
                "true" => true,
                "false" => false,
                _ => false,
            }) {
                set_random_playback.set(random_playback);
            }

            if let Ok(volume_js_val) =
                serde_wasm_bindgen::from_value::<String>(store.get("volume").await)
            {
                if let Ok(volume) = volume_js_val.parse::<f32>() {
                    set_volume.set(volume);
                }
            }

            if let Ok(js_val_string) =
                serde_wasm_bindgen::from_value::<String>(store.get("grid_data").await)
            {
                let grid_data = serde_json::from_str::<Vec<Option<Sample>>>(js_val_string.as_str());

                set_grid_data.set(grid_data.unwrap());
            } else {
                let mut grid_data_initial = vec![None; usize::from(DEFAULT_GRID_SIZE * 6)];
                fill_grid_initial(&mut grid_data_initial);
                set_grid_data.set(grid_data_initial);
            }

            set_save_blocked.set(false);
        });
    });

    // NOTE: Save state
    create_effect(move |_| {
        if save_blocked.get() {
            return;
        };
        let l_duration = gap_duration.get().to_string();
        let l_volume = volume.get().to_string();
        let l_random = random_playback.get().to_string();
        let l_grid_data = serde_json::to_value(grid_data.get()).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let store = Store::new("foo.bin");

            store.set("duration", l_duration.as_str()).await;
            store.set("volume", l_volume.as_str()).await;
            store.set("random_playback", l_random.as_str()).await;
            store
                .set("grid_data", l_grid_data.to_string().as_str())
                .await;
            store.save().await;
        });
    });

    // NOTE: Stop playing grid preview when opening sound lib
    create_effect(move |_| {
        edit_cell_idx.track();
        let secondary_audio_elem = secondary_audio_elem_ref
            .get()
            .expect("Failed to get ref to secondary audio element");
        if !secondary_audio_elem.paused() {
            let _ = secondary_audio_elem.pause();
            secondary_audio_elem.set_current_time(0.0);
        }
    });

    let grid_size_handler = move |op: Operation| {
        let mut gd = grid_data.get();
        let len = gd.len();

        // NOTE: Don't do anything if restriction boundaries are reached
        if op == Operation::Dec && len as u16 == GRID_ROWS_MIN * GRID_COLUMN_STEP
            || op == Operation::Inc && len as u16 == GRID_COLUMN_STEP * GRID_ROWS_MAX
        {
            return;
        }

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
                        // NOTE: Exclude possibility of duplicating random index
                        let current = thread_rng().gen_range(0..len);
                        if *val != current {
                            current
                        } else {
                            let middle = len / 2;
                            if *val > middle {
                                thread_rng().gen_range(0..(*val - 1))
                            } else {
                                thread_rng().gen_range((*val + 1)..len)
                            }
                        }
                    } else if *val == len - 1 {
                        0
                    } else {
                        *val + 1
                    }
                })
            },
            Duration::from_millis(gap_duration.get()),
        )
        .ok();

        set_timeout_handler.set(handler);
    };

    create_effect(move |_| {
        let main_audio_elem = main_audio_elem_ref
            .get()
            .expect("Failed to get ref to main audio element");
        let secondary_audio_elem = secondary_audio_elem_ref
            .get()
            .expect("Failed to get ref to secondary audio element");

        if play.get() {
            let _ = secondary_audio_elem.pause();
            secondary_audio_elem.set_current_time(0.0);

            if let Some(sample_opt) = grid_data.get().get(current_cell.get()) {
                if let Some(sample) = sample_opt {
                    main_audio_elem.set_src(&sample.filepath);

                    if let Ok(promise) = main_audio_elem.play() {
                        let reject_handler = Closure::new(move |err| {
                            logging::error!("{:?}", err);
                        });
                        let _ = promise.catch(&reject_handler);
                        reject_handler.forget();
                    }
                } else {
                    main_audio_elem.set_src(EMPTY_SOUND);
                    if let Ok(promise) = main_audio_elem.play() {
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
        } else {
            let _ = main_audio_elem.pause();
            main_audio_elem.set_current_time(0.0);
            if let Some(timeout_handle) = timeout_handler.get() {
                timeout_handle.clear();
                set_timeout_handler.set(None);
            }
        }
    });

    let grid_cell_click_handler =
        Callback::new(move |(sound_url_opt, idx): (Option<String>, u16)| {
            if !play.get() {
                if let Some(sound_url) = sound_url_opt {
                    let audio = secondary_audio_elem_ref
                        .get()
                        .expect("Failed to get ref to secondary audio element");

                    audio.set_src(&sound_url);

                    if let Ok(promise) = audio.play() {
                        let reject_handler = Closure::new(move |err| {
                            logging::error!("{:?}", err);
                        });
                        let _ = promise.catch(&reject_handler);
                        reject_handler.forget();
                    }
                }
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
        <div>
            <Grid
                grid_data
                current_cell
                click_handler=grid_cell_click_handler
                open_library_handler
                play
            />
            <ControlPanel
                play
                set_play
                gap_duration
                set_gap_duration
                grid_rows_num=Signal::derive(move || {
                    grid_data.get().len() as u16 / DEFAULT_GRID_SIZE
                })

                grid_size_handler
                volume
                set_volume
                random_playback
                set_random_playback
                set_presets_visible
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
            <Presets presets_visible />
            <audio _ref=main_audio_elem_ref prop:volume=volume on:ended=ended_listener></audio>
            <audio _ref=secondary_audio_elem_ref prop:volume=volume></audio>
        </div>
    }
}

fn fill_grid_initial(grid_data_initial: &mut [Option<Sample>]) {
    let sample = Sample {
        id: "boom_hit_1".to_string(),
        filepath: format!("{SOUND_LIB_PATH}boom/hit_1.mp3"),
        category: Category::Boom,
        filename: "hit_1".to_string(),
        duration: 0.32567,
    };

    let mod_idx = [0, 2, 3, 12, 23, 34];

    for (idx, item) in grid_data_initial.iter_mut().enumerate() {
        if mod_idx.contains(&idx) {
            *item = Some(sample.clone());
        }
    }
}
