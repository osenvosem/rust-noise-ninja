use std::collections::HashMap;

use crate::shared::{get_category_emoji, Category, Sample};
use ev::MouseEvent;
use html::Audio;
use leptos::*;
use leptos_heroicons::size_24::outline::SpeakerWave;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
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
                audio.set_src(&sample_path);
                let _ = audio.play();
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
        if let Some(sample_path) = target_elem.get_attribute("data-sample-filepath") {
            sample_select_handler(Sample {
                id: String::new(),
                filepath: sample_path,
                category: get_category_by_str(&target_elem.get_attribute("data-category").unwrap()),
                filename: target_elem.get_attribute("data-filename").unwrap(),
                duration: 0.0,
            });
        }
    };

    let render_view = sound_lib.into_iter().map(|(category, samples)| {
            // logging::log!("{:?}", samples);
            view! {
                <div class="mb-4">
                    <h2 class="select-none cursor-default mb-1">
                        {get_category_emoji(category.clone())} {category.to_string().to_uppercase()}
                    </h2>

                    <div class="flex flex-wrap">{
                        samples.iter().map(|sample| {
                            view! {
                                <div class="flex flex-col align-center justify-start mr-2">
                                    <div
                                        class="relative w-16 h-16 border-2 border-slate-400 rounded-full flex items-center justify-center select-none cursor-pointer hover:border-slate-600 font-bold mb-2"
                                        data-sample-filepath=&sample.filepath
                                        data-category=category.to_string()
                                        data-filename=&sample.filename
                                        on:click=sample_click_handler.clone()
                                    >
                                        <SpeakerWave class="w-10 h-10 top-2 right-2 bottom-2 left-2 stroke-slate-400 pointer-events-none"/>
                                    </div>
                                    <div class="w-16 text-xs text-slate-600 pointer-events-none text-center">
                                        {
                                                let sfn = sample.filename.clone();
                                                let formatted_filename = format!("{}{}", (sfn[..1]).to_uppercase(), &sfn[1..]).replace("_", " ");
                                                format!("{formatted_filename} ({:.2} sec)", sample.duration)
                                        }
                                    </div>
                                </div>
                            }
                        }).collect_view()
                    }</div>
                </div>
            }
    }).collect_view();

    view! {
        <div
            class=container_class
            style:display=move || {
                if edit_cell_idx.get().is_some() { "block" } else { "none" }
            }
        >

            <h1 class="absolute top-2 left-2 text-sm">You are editing cell # {edit_cell_idx}</h1>
            <div class="mt-12 mb-16 px-12" on:dblclick=sample_double_click_handler>
                {render_view}
            </div>
            <ControlPanel
                on_close=close_library_handler
                on_clear_cell=clear_cell_handler
                is_cell_filled
            />
            <audio _ref=audio_ref prop:volume=volume></audio>
        </div>
    }
}

fn get_category_by_str(s: &str) -> Category {
    match s {
        "boom" => Category::Boom,
        "doors" => Category::Doors,
        "construction" => Category::Construction,
        "eerie" => Category::Eerie,
        _ => Category::Boom,
    }
}

#[component]
fn ControlPanel(
    on_close: Callback<MouseEvent>,
    on_clear_cell: Callback<MouseEvent>,
    is_cell_filled: Signal<bool>,
) -> impl IntoView {
    let container = "fixed bottom-[4%] w-screen h-[56px]";
    let container_inner = "w-60 h-[100%] mx-auto flex items-center justify-center";
    let button_class =
        "border-0 bg-white shadow-md border-slate-400 rounded-lg p-1 hover:border-2 w-20 h-12 mr-4 text:slate-600 text-sm";

    view! {
        <div class=container>
            <div class=container_inner>
                <button
                    class=move || {
                        format!(
                            "{button_class} {}",
                            if is_cell_filled.get() { "" } else { "hidden" },
                        )
                    }

                    on:click=on_clear_cell
                >
                    "Clear cell"
                </button>
                <button class=button_class on:click=on_close>
                    "Close"
                </button>
            </div>
        </div>
    }
}
