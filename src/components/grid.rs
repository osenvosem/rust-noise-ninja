use crate::shared::{format_filename, get_category_emoji, Sample};
use ev::MouseEvent;
use leptos::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use web_sys::HtmlDivElement;

#[component]
pub fn Grid(
    grid_data: ReadSignal<Vec<Option<Sample>>>,
    current_cell: ReadSignal<usize>,
    click_handler: Callback<(Option<String>, u16)>,
    #[prop(into)] open_library_handler: Callback<u16>,
) -> impl IntoView {
    let container_class = "p-6 grid grid-cols-6 gap-2 pb-20";
    let item_class = "h-16 rounded shadow-sm flex justify-center items-center hover:cursor-pointer hover:border-2 hover:shadow-lg active:shadow-sm bg-white/80 backdrop-blur-md";
    let item_active_class = "border-2 border-amber-400";
    let content_class = "flex flex-col items-center pointer-events-none select-none text-xs";

    let UseTimeoutFnReturn {
        start,
        stop,
        is_pending,
        ..
    } = use_timeout_fn(
        move |e: MouseEvent| {
            let target = event_target::<HtmlDivElement>(&e);
            if let Some(sound_url) = target.get_attribute("data-sound-url") {
                let idx = target
                    .get_attribute("data-idx")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap();
                let arg = if sound_url.is_empty() {
                    (None, idx)
                } else {
                    (Some(sound_url), idx)
                };
                click_handler(arg);
            }
        },
        300.0,
    );

    let local_click_handler = move |e: MouseEvent| {
        if !is_pending.get() {
            start(e);
        }
    };

    let double_click_handler = move |e: MouseEvent| {
        if is_pending.get() {
            stop();
        }
        let target = event_target::<HtmlDivElement>(&e);
        if let Some(idx) = target.get_attribute("data-idx") {
            open_library_handler(idx.parse::<u16>().unwrap());
        }
    };

    let grid_elems = move || {
        grid_data
            .get()
            .into_iter()
            .enumerate()
            .map(|(idx, elem)| {
                view! {
                    <div
                        data-sound-url=if let Some(sample) = elem.clone() {
                            sample.filepath
                        } else {
                            "".to_string()
                        }

                        data-idx=idx
                        class=move || {
                            format!(
                                "{item_class} {} {}",
                                if idx == current_cell.get() { item_active_class } else { "" },
                                if idx != current_cell.get() { "hover:border-white" } else { "" },
                            )
                        }
                    >
                        {if let Some(sample) = elem {
                            let icon = get_category_emoji(sample.category);
                            let filename = format_filename(&sample.filename);
                            let duration = format!(
                                "{:.2}s",
                                &sample.duration
                            );


                            view! {
                                <div class=content_class>
                                    <div>{icon}</div>
                                    <div class="font-semibold">{filename}</div>
                                    <div>{duration}</div>
                                </div>
                            }.into_view()
                        } else {
                            view! { "" }.into_view()
                        }}

                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <div class=container_class on:click=local_click_handler on:dblclick=double_click_handler>
            {grid_elems}
        </div>
    }
}
