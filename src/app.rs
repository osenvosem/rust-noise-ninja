use crate::components::{
    control_panel::ControlPanel, grid::Grid, presets::Presets, schedule::Schedule,
    settings_menu::SettingsMenu, sound_library::SoundLibrary,
};
use crate::shared::{
    grid_row_size, Category, Operation, PlannedSchedule, Preset, RecurringSchedule, Sample,
    ScheduleType, EMPTY_SOUND, GRID_ROWS_MAX, GRID_ROWS_MIN, SOUND_LIB_JSON_PATH, SOUND_LIB_PATH,
};
use chrono::{Datelike, Local, Utc};
use html::Audio;
use leptos::{error::ErrorBoundary, prelude::*, *};
use leptos_dom::helpers::TimeoutHandle;
use leptos_use::{
    use_timestamp_with_controls_and_options, UseTimestampOptions, UseTimestampReturn,
};
use rand::distr::{Alphanumeric, SampleString};
use rand::{rng, Rng};
use std::collections::HashMap;
use std::time::Duration;
use wasm_bindgen::{closure::Closure, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __TAURI_PLUGIN_STORE__)]
    async fn load(filename: &str) -> Store;
}

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

    #[wasm_bindgen(method)]
    async fn values(this: &Store) -> JsValue;

    #[wasm_bindgen(method)]
    async fn entries(this: &Store) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (grid_data, set_grid_data) = signal::<Vec<Option<Sample>>>(Vec::new());
    let (play, set_play) = signal(false);
    let (gap_duration, set_gap_duration) = signal(1000);
    let (current_cell, set_current_cell) = signal(0);
    let (timeout_handler, set_timeout_handler) = signal::<Option<TimeoutHandle>>(None);
    let (volume, set_volume) = signal::<f32>(1.0);
    let (edit_cell_idx, set_edit_cell_idx) = signal::<Option<u16>>(None);
    let (random_playback, set_random_playback) = signal(false);
    let (save_blocked, set_save_blocked) = signal(false);
    let (presets_visible, set_presets_visible) = signal(false);
    let (presets, set_presets) = signal::<Vec<Preset>>(Vec::new());
    let (scheduled_playback, set_scheduled_playback) = signal(false);
    let (schedule_visible, set_schedule_visible) = signal(false);
    let (planned_schedules, set_planned_schedules) = signal::<Vec<PlannedSchedule>>(Vec::new());
    let (recurring_schedules, set_recurring_schedules) =
        signal::<Vec<RecurringSchedule>>(Vec::new());
    let (schedule_type, set_schedule_type) = signal(ScheduleType::Reccuring);

    let sound_lib = LocalResource::new(|| async {
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
    });

    let main_audio_elem_ref = NodeRef::<Audio>::new();
    let secondary_audio_elem_ref = NodeRef::<Audio>::new();

    let UseTimestampReturn {
        timestamp,
        is_active,
        pause,
        resume,
    } = use_timestamp_with_controls_and_options(UseTimestampOptions::default().interval(1000));

    // NOTE: moved here in order to use the closure in scheduled playback effect
    let load_preset_handler = Callback::new(move |preset: Preset| {
        let Preset {
            gap_duration,
            volume,
            random_playback,
            grid_data,
            ..
        } = preset;

        set_gap_duration(gap_duration);
        set_volume(volume);
        set_random_playback(random_playback);
        set_grid_data(grid_data);

        set_presets_visible(false);
    });

    // NOTE: Restore state
    Effect::new(move |_| {
        set_save_blocked(true);

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;

            // store.clear().await;
            if let Ok(gap_duration_js_val) =
                serde_wasm_bindgen::from_value::<String>(store.get("duration").await)
            {
                if let Ok(gap_duration) = gap_duration_js_val.parse::<u64>() {
                    set_gap_duration(gap_duration);
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
                set_random_playback(random_playback);
            }

            if let Ok(volume_js_val) =
                serde_wasm_bindgen::from_value::<String>(store.get("volume").await)
            {
                if let Ok(volume) = volume_js_val.parse::<f32>() {
                    set_volume(volume);
                }
            }

            if let Ok(grid_data_js_val) =
                serde_wasm_bindgen::from_value::<String>(store.get("grid_data").await)
            {
                let grid_data =
                    serde_json::from_str::<Vec<Option<Sample>>>(grid_data_js_val.as_str());

                set_grid_data(grid_data.unwrap());
            } else {
                let mut grid_data_initial = vec![None; usize::from(grid_row_size() * 2)];
                fill_grid_initial(&mut grid_data_initial);
                set_grid_data(grid_data_initial);
            }

            set_save_blocked(false);
        });
    });

    // NOTE: Save state
    Effect::new(move |_| {
        if save_blocked() {
            return;
        };
        let l_duration = gap_duration().to_string();
        let l_volume = volume().to_string();
        let l_random = random_playback().to_string();
        let l_grid_data = serde_json::to_string(&grid_data()).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;

            store.set("duration", l_duration.as_str()).await;
            store.set("volume", l_volume.as_str()).await;
            store.set("random_playback", l_random.as_str()).await;
            store.set("grid_data", l_grid_data.as_str()).await;
            store.save().await;
        });
    });

    // NOTE: Stop playing grid preview when opening sound lib
    Effect::watch(
        edit_cell_idx,
        move |_, _, _| {
            let secondary_audio_elem = secondary_audio_elem_ref
                .get()
                .expect("Failed to get ref to secondary audio element");
            if !secondary_audio_elem.paused() {
                let _ = secondary_audio_elem.pause();
                secondary_audio_elem.set_current_time(0.0);
            }
        },
        true,
    );

    // NOTE: Restore presets
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;

            let keys_vec_res = serde_wasm_bindgen::from_value::<Vec<String>>(store.keys().await);

            if let Ok(keys) = keys_vec_res {
                let presets_vec_fut = keys
                    .iter()
                    .filter(|key| key.starts_with("preset_"))
                    .map(|key| async {
                        let preset_string =
                            serde_wasm_bindgen::from_value::<String>(store.get(key).await).unwrap();
                        serde_json::from_str::<Preset>(preset_string.as_str()).unwrap()
                    })
                    .collect::<Vec<_>>();

                let mut stored_presets = futures::future::join_all(presets_vec_fut).await;
                stored_presets.sort();
                set_presets(stored_presets);
            }
        });
    });

    // NOTE: Play grid
    Effect::new(move |_| {
        let main_audio_elem = main_audio_elem_ref
            .get()
            .expect("Failed to get ref to main audio element");
        let secondary_audio_elem = secondary_audio_elem_ref
            .get()
            .expect("Failed to get ref to secondary audio element");

        if play() {
            let _ = secondary_audio_elem.pause();
            secondary_audio_elem.set_current_time(0.0);

            if let Some(sample_opt) = grid_data().get(current_cell()) {
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
            if let Some(timeout_handle) = timeout_handler() {
                timeout_handle.clear();
                set_timeout_handler(None);
            }
        }
    });

    // NOTE: scheduled playback
    Effect::watch(
        timestamp,
        move |_, _, _| {
            if scheduled_playback() {
                if !is_active() {
                    resume();
                };

                match schedule_type() {
                    ScheduleType::Planned => {
                        let current_dt = Local::now().naive_local();
                        let schedules = planned_schedules();
                        let current_schedule = schedules
                            .iter()
                            .find(|s| current_dt >= s.start && current_dt < s.end);

                        if let Some(s) = current_schedule {
                            if !play() {
                                load_preset_handler.run(s.preset.clone());
                                set_play(true);
                            }
                        } else if play() {
                            set_play(false);
                        }
                    }
                    ScheduleType::Reccuring => {
                        let current_t = Local::now().time();
                        let schedules = recurring_schedules();
                        let current_schedule = schedules.iter().find(|s| {
                            s.weekdays.contains(&Local::now().weekday())
                                && current_t >= s.start
                                && current_t < s.end
                        });

                        if let Some(s) = current_schedule {
                            if !play() {
                                load_preset_handler.run(s.preset.clone());
                                set_play(true);
                            }
                        } else if play() {
                            set_play(false);
                        }
                    }
                }
            } else {
                pause();
            };
        },
        true,
    );

    // NOTE: Restore schedules
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;

            let planned_schedules_str_result =
                serde_wasm_bindgen::from_value::<String>(store.get("planned_schedules").await);

            if let Ok(schedules_str) = planned_schedules_str_result {
                let stored_schedules =
                    serde_json::from_str::<Vec<PlannedSchedule>>(&schedules_str).unwrap();

                set_planned_schedules(stored_schedules);
            }

            let recurring_schedules_str_result =
                serde_wasm_bindgen::from_value::<String>(store.get("recurring_schedules").await);

            if let Ok(schedules_str) = recurring_schedules_str_result {
                let stored_schedules =
                    serde_json::from_str::<Vec<RecurringSchedule>>(&schedules_str).unwrap();

                set_recurring_schedules(stored_schedules);
            }
        });
    });

    let grid_size_handler = Callback::new(move |op: Operation| {
        let mut gd = grid_data();
        let len = gd.len();

        // NOTE: Don't do anything if restriction boundaries are reached
        if op == Operation::Dec && len as u16 == GRID_ROWS_MIN * grid_row_size()
            || op == Operation::Inc && len as u16 == grid_row_size() * GRID_ROWS_MAX
        {
            return;
        }

        set_grid_data(match op {
            Operation::Dec => {
                gd.drain(len - grid_row_size() as usize..);
                gd
            }
            Operation::Inc => {
                gd.splice(len.., vec![None; grid_row_size() as usize]);
                gd
            }
        });
    });

    let ended_listener = move |_| {
        let handler = set_timeout_with_handle(
            move || {
                set_current_cell.update(move |val| {
                    let len = grid_data().len();
                    *val = if random_playback() {
                        // NOTE: Exclude possibility of duplicating random index
                        let current = rng().random_range(0..len);
                        if *val != current {
                            current
                        } else {
                            let middle = len / 2;
                            if *val > middle {
                                rng().random_range(0..(*val - 1))
                            } else {
                                rng().random_range((*val + 1)..len)
                            }
                        }
                    } else if *val == len - 1 {
                        0
                    } else {
                        *val + 1
                    }
                })
            },
            Duration::from_millis(gap_duration()),
        )
        .ok();

        set_timeout_handler(handler);
    };

    let grid_cell_click_handler =
        Callback::new(move |(sound_url_opt, idx): (Option<String>, u16)| {
            if !play.get() {
                if let Some(sound_url) = sound_url_opt {
                    let audio = secondary_audio_elem_ref
                        .get()
                        .expect("Failed to get ref to secondary audio element");

                    if !audio.paused() && audio.src().contains(&sound_url) {
                        let _ = audio.pause();
                        audio.set_current_time(0.0);
                    } else {
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
            }

            set_current_cell.set(idx as usize);
        });

    let sample_select_handler = Callback::new(move |sample| {
        let idx = edit_cell_idx().unwrap();
        set_edit_cell_idx(None);
        let mut mut_grid_data = grid_data();
        mut_grid_data[idx as usize] = Some(sample);
        set_grid_data(mut_grid_data);
    });

    let open_library_handler = Callback::new(move |idx| {
        set_edit_cell_idx(Some(idx));
    });

    let close_library_handler = Callback::new(move |_| {
        set_edit_cell_idx(None);
    });

    let clear_cell_handler = Callback::new(move |_| {
        let idx = edit_cell_idx().unwrap();
        set_edit_cell_idx(None);
        let mut mut_grid_data = grid_data();
        mut_grid_data[idx as usize] = None;
        set_grid_data(mut_grid_data);
    });

    let save_preset_handler = Callback::new(move |preset_name: String| {
        let time = Utc::now();
        let preset = Preset {
            id: format!("preset_{}", Alphanumeric.sample_string(&mut rand::rng(), 4)),
            name: if preset_name.is_empty() {
                format!("Preset {}", time.format("%Y.%m.%d %H:%M"))
            } else {
                preset_name
            },
            volume: volume(),
            gap_duration: gap_duration(),
            random_playback: random_playback(),
            grid_data: grid_data(),
            created: time,
        };

        set_presets.update(|p| p.push(preset.clone()));

        let preset_str = serde_json::to_value(&preset).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;
            store.set(preset.id.as_str(), &preset_str.to_string()).await;
        });
    });

    let delete_preset_handler = Callback::new(move |key: String| {
        let cloned_key = key.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;
            let _ = store.delete(cloned_key.as_str()).await;
        });

        set_presets.update(|prs| {
            *prs = prs
                .iter()
                .filter(|ps| ps.id != key)
                .cloned()
                .collect::<Vec<Preset>>();
        });
    });

    let save_planned_schedule_handler = Callback::new(move |schedule: PlannedSchedule| {
        set_planned_schedules.update(|s| {
            s.push(schedule);
        });

        let schedules_str = serde_json::to_value(planned_schedules.get()).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;
            store
                .set("planned_schedules", &schedules_str.to_string())
                .await;
        });
    });

    let delete_planned_schedule_handler = Callback::new(move |schedule_id: String| {
        let updated = planned_schedules
            .get()
            .iter()
            .filter(|s| s.id != schedule_id)
            .cloned()
            .collect::<Vec<PlannedSchedule>>();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;

            store
                .set(
                    "planned_schedules",
                    &serde_json::to_value(&updated).unwrap().to_string(),
                )
                .await;

            set_planned_schedules.update(|s| {
                *s = updated;
            });
        });
    });

    let save_recurring_schedule_handler = Callback::new(move |schedule: RecurringSchedule| {
        set_recurring_schedules.update(|s| {
            s.push(schedule);
        });

        let schedules_str = serde_json::to_value(recurring_schedules.get()).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;
            store
                .set("recurring_schedules", &schedules_str.to_string())
                .await;
        });
    });

    let delete_recurring_schedule_handler = Callback::new(move |schedule_id: String| {
        let updated = recurring_schedules
            .get()
            .iter()
            .filter(|s| s.id != schedule_id)
            .cloned()
            .collect::<Vec<RecurringSchedule>>();

        wasm_bindgen_futures::spawn_local(async move {
            let store = load("store.bin").await;

            store
                .set(
                    "recurring_schedules",
                    &serde_json::to_value(&updated).unwrap().to_string(),
                )
                .await;

            set_recurring_schedules.update(|s| {
                *s = updated;
            });
        });
    });

    // NOTE: Erase grid
    let erase_grid_handler = Callback::new(move |_: ev::MouseEvent| {
        set_grid_data.update(|grid| {
            *grid = grid.iter_mut().map(|_| None).collect();
        });
    });

    let is_cell_filled = Signal::derive(move || {
        if let Some(idx) = edit_cell_idx() {
            grid_data().get(idx as usize).unwrap().is_some()
        } else {
            false
        }
    });

    let is_schedules_empty = Signal::derive(move || {
        if schedule_type() == ScheduleType::Planned {
            planned_schedules().is_empty()
        } else {
            recurring_schedules().is_empty()
        }
    });

    view! {
        <div>
            <SettingsMenu
                gap_duration
                set_gap_duration
                grid_size_handler
                grid_rows_num=Signal::derive(move || {
                    grid_data.get().len() as u16 / grid_row_size()
                })
                set_presets_visible
                set_schedule_visible
                erase_grid_handler
            />

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
                volume
                set_volume
                random_playback
                set_random_playback
                scheduled_playback
                set_scheduled_playback
                is_schedules_empty
            />
            <Suspense fallback=move || view! { "" }>
                <ErrorBoundary fallback=|_| {
                    view! { <p>"Something went wrong"</p> }
                }>
                    {move || {
                        sound_lib
                            .get()
                            .map(|lib| {
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
            <Presets
                presets_visible
                set_presets_visible
                save_preset_handler
                presets
                delete_preset_handler
                load_preset_handler
            />
            <Schedule
                schedule_visible
                set_schedule_visible
                planned_schedules
                save_planned_schedule=save_planned_schedule_handler
                delete_planned_schedule=delete_planned_schedule_handler
                recurring_schedules
                save_recurring_schedule=save_recurring_schedule_handler
                delete_recurring_schedule=delete_recurring_schedule_handler
                schedule_type
                set_schedule_type
                presets
            />

            <audio node_ref=main_audio_elem_ref prop:volume=volume on:ended=ended_listener></audio>
            <audio node_ref=secondary_audio_elem_ref prop:volume=volume></audio>
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

    let mod_idx = [0, 2, 4, 6, 8, 10];

    for (idx, item) in grid_data_initial.iter_mut().enumerate() {
        if mod_idx.contains(&idx) {
            *item = Some(sample.clone());
        }
    }
}
