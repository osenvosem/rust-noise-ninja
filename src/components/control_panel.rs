use leptos::*;
use leptos_heroicons::size_24::outline::{
    ArrowsRightLeft, Clock, PauseCircle, PlayCircle, SpeakerWave, SpeakerXMark,
};
use leptos_use::on_click_outside;
use web_sys::SvgElement;

#[component]
pub fn ControlPanel(
    play: ReadSignal<bool>,
    set_play: WriteSignal<bool>,
    volume: ReadSignal<f32>,
    set_volume: WriteSignal<f32>,
    random_playback: ReadSignal<bool>,
    set_random_playback: WriteSignal<bool>,
    scheduled_playback: ReadSignal<bool>,
    set_scheduled_playback: WriteSignal<bool>,
    is_schedules_empty: Signal<bool>,
) -> impl IntoView {
    let outer_container_class = "fixed bottom-[2%] left-[2%] right-[2%]";
    let inner_container_class =
        "fixed bottom-[2%] left-[2%] right-[2%] flex h-[56px] w-[80%] sm:w-[50%] left-[50%] translate-x-[-50%] bg-white/80 drop-shadow-md rounded-full backdrop-blur-md";
    let left_container_class = "flex flex-1 items-center justify-start pl-[6%]";
    let center_container_class = "flex flex-1 items-center justify-center relative";
    let right_container_class = "flex flex-1 items-center justify-end pr-[6%]";

    view! {
        <div class=outer_container_class>
            <div class=inner_container_class>
                <div class=left_container_class>
                    <RandomPlaybackButton random_playback set_random_playback />
                </div>
                <div class=center_container_class>
                    <PlayButton
                        play
                        set_play
                        scheduled_playback
                        set_scheduled_playback
                        is_schedules_empty
                    />
                </div>
                <div class=right_container_class>
                    <VolumeControl volume set_volume />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PlayButton(
    play: ReadSignal<bool>,
    set_play: WriteSignal<bool>,
    scheduled_playback: ReadSignal<bool>,
    set_scheduled_playback: WriteSignal<bool>,
    is_schedules_empty: Signal<bool>,
) -> impl IntoView {
    let button_class = "w-12 h-12 rounded-full relative";
    let icon_class = "stroke-slate-950 stroke-1";

    view! {
        <div class="flex items-center">
            <button
                class=move || {
                    format!(
                        "{button_class} {}",
                        if scheduled_playback.get() { "opacity-30" } else { "" },
                    )
                }
                on:click=move |_| {
                    set_play
                        .update(|val| {
                            *val = !*val;
                        })
                }
                disabled=move || scheduled_playback.get()
            >

                <Show
                    when=move || { play.get() }
                    fallback=move || {
                        view! { <PlayCircle class=icon_class /> }
                    }
                >

                    <PauseCircle class=icon_class />
                </Show>
            </button>
            <button
                class=move || {
                    format!(
                        "flex mr-4 cursor-pointer {}",
                        if is_schedules_empty.get() { "opacity-30" } else { "" },
                    )
                }
                on:click=move |_| { set_scheduled_playback.update(|val| *val = !*val) }
                disabled=is_schedules_empty
            >
                <Clock class=move || {
                    format!(
                        "w-6 h-6 cursor-pointer{}",
                        if scheduled_playback.get() {
                            " stroke-blue-500"
                        } else {
                            " stroke-slate-950"
                        },
                    )
                } />
            </button>
        </div>
    }
}

#[component]
pub fn VolumeControl(volume: ReadSignal<f32>, set_volume: WriteSignal<f32>) -> impl IntoView {
    let container_class = "relative flex flex-col w-6 h-6";
    let icon_class = "cursor-pointer stroke-slate-950";
    let input_container =
        "absolute top-[-120px] left-[-68px] bg-white shadow rounded-full p-4 -rotate-90";
    let input_class =
        "block w-32 h-1 bg-slate-950 rounded-lg appearance-none cursor-pointer volume-thumb";

    let (open, set_open) = create_signal(false);

    let input_container_ref = create_node_ref();

    let _ = on_click_outside(input_container_ref, move |e| {
        let target = event_target::<SvgElement>(&e);
        let tag = target.tag_name();
        let tags = ["svg", "path"];
        if open.get() && !tags.contains(&tag.as_str()) {
            set_open.set(false);
        }
    });

    view! {
        <div class=container_class>
            <div data-icon-container on:click=move |_| set_open.update(|val| *val = !*val)>
                <Show
                    when=move || volume.get() != 0.0
                    fallback=move || {
                        view! { <SpeakerXMark class=icon_class /> }
                    }
                >
                    <SpeakerWave class=icon_class />
                </Show>
            </div>
            <div
                _ref=input_container_ref
                class=move || {
                    format!("{input_container}{}", if open.get() { "" } else { " hidden" })
                }
            >
                <input
                    id="volume-range"
                    type="range"
                    value=move || { volume.get() * 100.0 }
                    min=0
                    max=100
                    class=input_class
                    on:change=move |e| {
                        set_volume.set(event_target_value(&e).parse::<f32>().unwrap() / 100.0)
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn RandomPlaybackButton(
    random_playback: ReadSignal<bool>,
    set_random_playback: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div
            class="w-6 h-6 flex mr-4 cursor-pointer"
            on:click=move |_| { set_random_playback.update(|val| *val = !*val) }
        >
            <ArrowsRightLeft class=move || {
                format!(
                    "cursor-pointer{}",
                    if random_playback.get() { " stroke-blue-500" } else { " stroke-slate-950" },
                )
            } />
        </div>
    }
}
