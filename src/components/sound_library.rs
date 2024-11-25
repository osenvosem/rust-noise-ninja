use crate::components::button::Button;
use crate::shared::{format_filename, Category, Sample};
use ev::MouseEvent;
use html::Audio;
use leptos::*;
use leptos_heroicons::size_24::outline::SpeakerWave;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use std::collections::HashMap;
use std::str::FromStr;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlDivElement;

#[component]
pub fn SoundLibrary(
    sound_lib: HashMap<Category, Vec<Sample>>,
    edit_cell_idx: ReadSignal<Option<u16>>,
    is_cell_filled: Signal<bool>,
    #[prop(into)] sample_select_handler: Callback<Sample>,
    #[prop(into)] close_library_handler: Callback<MouseEvent>,
    #[prop(into)] clear_cell_handler: Callback<MouseEvent>,
    volume: ReadSignal<f32>,
) -> impl IntoView {
    let local_sound_lib = sound_lib.clone();
    let container_class =
        "absolute top-0 right-0 bottom-auto left-0 min-h-screen w-screen bg-white ";
    let audio_ref = create_node_ref::<Audio>();

    let UseTimeoutFnReturn {
        start,
        stop,
        is_pending,
        ..
    } = use_timeout_fn(
        move |e: MouseEvent| {
            let audio = audio_ref
                .get()
                .expect("Failed to get ref to lib audio element");
            let target_elem = event_target::<HtmlDivElement>(&e);

            if let Some(sample_path) = target_elem.get_attribute("data-sample-filepath") {
                let current_sample_path = audio.src();

                if !audio.paused() && current_sample_path.contains(&sample_path) {
                    let _ = audio.pause();
                    audio.set_current_time(0.0);
                } else {
                    audio.set_src(&sample_path);
                    if let Ok(promise) = audio.play() {
                        let reject_handler = Closure::new(move |err| {
                            logging::error!("{:?}", err);
                        });
                        let _ = promise.catch(&reject_handler);
                        reject_handler.forget();
                    }
                }
            }
        },
        200.0,
    );

    let sample_click_handler = move |e: MouseEvent| {
        if !is_pending.get() {
            start(e);
        }
    };

    let sample_double_click_handler = move |e: MouseEvent| {
        if is_pending.get() {
            stop();
        }

        let target_elem = event_target::<HtmlDivElement>(&e);
        if let Some(sample_id) = target_elem.get_attribute("data-sample-id") {
            if let Some(category_str) = target_elem.get_attribute("data-category") {
                if let Ok(category) = Category::from_str(category_str.as_str()) {
                    let audio = audio_ref
                        .get()
                        .expect("Failed to get ref to lib audio element");

                    let category_vec = local_sound_lib.get(&category).unwrap();

                    let sample = category_vec
                        .iter()
                        .find(|&sample| sample.id == sample_id)
                        .unwrap();
                    sample_select_handler(sample.clone());
                    let _ = audio.pause();
                    audio.set_current_time(0.0);
                }
            }
        }
    };

    let close_library_local_handler = Callback::new(move |e: MouseEvent| {
        let audio = audio_ref
            .get()
            .expect("Failed to get ref to lib audio element");

        let _ = audio.pause();
        audio.set_current_time(0.0);

        close_library_handler(e);
    });

    let render_view = Category::iter().map(|category| {
            let samples = sound_lib.get(&category).unwrap();
            view! {
                <div class="mb-2">
                    <h2 class="select-none cursor-default mb-4">
                        {category.get_emoji()} {category.to_string().to_uppercase()}
                    </h2>

                    <div class="flex flex-wrap">
                        {samples
                            .iter()
                            .map(|sample| {
                                view! {
                                    <div
                                        class="flex flex-col align-center justify-start mr-4 mb-4 cursor-pointer"
                                        data-sample-id=sample.id.clone()
                                        data-category=category.to_string()
                                        data-sample-filepath=&sample.filepath
                                        on:click=sample_click_handler.clone()
                                    >
                                        <div class="relative w-16 h-16 border-2 border-slate-400 rounded-full flex items-center justify-center select-none hover:border-slate-950 font-bold mb-2 pointer-events-none">
                                            <SpeakerWave class="w-10 h-10 top-2 right-2 bottom-2 left-2 stroke-slate-400 pointer-events-none" />
                                        </div>
                                        <div class="flex flex-col items-center text-xs text-slate-950 text-center pointer-events-none">
                                            <div class="font-semibold select-none pointer-events-none">
                                                {format_filename(&sample.filename)}
                                            </div>
                                            <div class="select-none pointer-events-none">
                                                {format!("{:.2}s", &sample.duration)}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            }
    }).collect_view();

    view! {
        <div
            class=container_class
            style:display=move || { if edit_cell_idx.get().is_some() { "block" } else { "none" } }
        >

            <h1 class="absolute top-2 left-2 text-sm select-none">
                You are editing cell #{edit_cell_idx}
            </h1>
            <div class="mt-12 mb-20 px-4 md:px-8" on:dblclick=sample_double_click_handler>
                {render_view}
            </div>
            <ControlPanel
                on_close=close_library_local_handler
                on_clear_cell=clear_cell_handler
                is_cell_filled
            />
            <audio _ref=audio_ref prop:volume=volume></audio>
        </div>
    }
}

#[component]
fn ControlPanel(
    on_close: Callback<MouseEvent>,
    on_clear_cell: Callback<MouseEvent>,
    is_cell_filled: Signal<bool>,
) -> impl IntoView {
    let container = "fixed bottom-[2%] w-screen h-[56px]";
    let container_inner = "w-60 h-[100%] mx-auto flex items-center justify-center";

    view! {
        <div class=container>
            <div class=container_inner>
                <Button
                    class="mr-4"
                    hidden=Signal::derive(move || !is_cell_filled.get())
                    on:click=on_clear_cell
                >
                    "Clear cell"
                </Button>
                <Button on_click=on_close>"Close"</Button>
            </div>
        </div>
    }
}
