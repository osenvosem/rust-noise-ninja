use crate::shared::{Operation, GRID_ROWS_MAX, GRID_ROWS_MIN};
use leptos::*;
use leptos_heroicons::size_24::outline::{
    ArrowsRightLeft, PauseCircle, PlayCircle, SpeakerWave, SpeakerXMark,
};
use leptos_use::on_click_outside;
use web_sys::{HtmlInputElement, SvgElement};

#[component]
pub fn ControlPanel(
    play: ReadSignal<bool>,
    set_play: WriteSignal<bool>,
    gap_duration: ReadSignal<u64>,
    set_gap_duration: WriteSignal<u64>,
    grid_rows_num: Signal<u16>,
    #[prop(into)] grid_size_handler: Callback<Operation>,
    volume: ReadSignal<f32>,
    set_volume: WriteSignal<f32>,
    random_playback: ReadSignal<bool>,
    set_random_playback: WriteSignal<bool>,
) -> impl IntoView {
    let outer_container_class = "fixed bottom-[2%] left-[2%] right-[2%]";
    let inner_container_class =
        "fixed bottom-[2%] left-[2%] right-[2%] flex h-[56px] bg-white/80 drop-shadow-md rounded-lg backdrop-blur-md";
    let left_container_class = "flex flex-1 items-center justify-start pl-[2%]";
    let center_container_class = "flex flex-1 items-center justify-center";
    let right_container_class = "flex flex-1 items-center justify-end pr-[2%]";

    view! {
        // the wrapper fixes control panel jumping when openning volume control
        <div class=outer_container_class>
            <div class=inner_container_class>
                <div class=left_container_class>
                    <PlaybackGapDuration gap_duration set_gap_duration/>
                    <GridSizeControl grid_rows_num grid_size_handler/>
                </div>
                <div class=center_container_class>
                    <PlayButton play set_play/>
                </div>
                <div class=right_container_class>
                    <RandomPlaybackButton random_playback set_random_playback/>
                    <VolumeControl volume set_volume/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PlayButton(play: ReadSignal<bool>, set_play: WriteSignal<bool>) -> impl IntoView {
    let button_class = "w-12 h-12 rounded-full";
    let icon_class = "stroke-slate-950 stroke-1";

    view! {
        <button
            class=button_class
            on:click=move |_| {
                set_play
                    .update(|val| {
                        *val = !*val;
                    })
            }
        >

            <Show
                when=move || { play.get() }
                fallback=move || {
                    view! { <PlayCircle class=icon_class/> }
                }
            >

                <PauseCircle class=icon_class/>
            </Show>
        </button>
    }
}

#[component]
pub fn VolumeControl(volume: ReadSignal<f32>, set_volume: WriteSignal<f32>) -> impl IntoView {
    let container_class = "relative flex flex-col w-6 h-6";
    let icon_class = "cursor-pointer stroke-slate-950";
    let input_container =
        "absolute top-[-120px] left-[-68px] bg-white shadow rounded-full p-4 -rotate-90";
    let input_class = "block w-32 h-1 bg-slate-950 rounded-lg appearance-none cursor-pointer";

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
                <Show when=move || volume.get() != 0.0 fallback=move || { view! {<SpeakerXMark class=icon_class/>} }>
                    <SpeakerWave class=icon_class/>
                </Show>
            </div>
            <div _ref=input_container_ref class=move || format!("{input_container}{}", if open.get() {""} else {" hidden"})>
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
pub fn PlaybackGapDuration(
    gap_duration: ReadSignal<u64>,
    set_gap_duration: WriteSignal<u64>,
) -> impl IntoView {
    let container_class = "flex flex-col";
    let inc_dec_handler = move |op: Operation| {
        let step = 500;
        set_gap_duration.update(|v| {
            *v = match op {
                Operation::Inc => *v + step,
                Operation::Dec => *v - step,
            }
        });
    };

    let dots_re = regex::Regex::new(r"\.+").unwrap();

    let input_handler = move |e: ev::Event| {
        let target: HtmlInputElement = event_target(&e);
        let mut val = event_target_value(&e);

        if !val.is_empty() && val.ends_with('.') {
            let replaced = dots_re.replace_all(&val, ".");
            target.set_value(&replaced);
        } else if let Ok(parsed) = val.parse::<f64>() {
            let float_mult = parsed * 1000.0;
            set_gap_duration.set(float_mult as u64);
        } else {
            val.pop();
            target.set_value(&val);
        }
    };

    view! {
        <div class=container_class>
            <label
                for="speed-input"
                class="block mb-1 text-xs font-medium text-slate-950 select-none"
            >
                "Silent gap (sec)"
            </label>
            <div class="relative flex items-center">
                <button
                    on:click=move |_| { inc_dec_handler(Operation::Dec) }
                    type="button"
                    id="decrement-button"
                    data-input-counter-decrement="counter-input"
                    class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none"
                    style=("opacity", move || { if gap_duration.get() == 0 { "0.6" } else { "1" } })
                    disabled=move || { gap_duration.get() == 0 }
                >
                    <svg
                        class="w-2.5 h-2.5 text-slate-950 pointer-events-none"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 18 2"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1h16"
                        ></path>
                    </svg>
                </button>
                <input
                    type="text"
                    id="speed-input"
                    data-input-counter
                    class="flex-shrink-0 text-slate-950 border-0 bg-transparent text-xs font-normal focus:outline-none focus:ring-0 max-w-[2.5rem] text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                    placeholder=""
                    prop:value=move || { gap_duration.get() as f64 / 1000_f64 }
                    on:input=input_handler
                />

                <button
                    on:click=move |_| { inc_dec_handler(Operation::Inc) }
                    type="button"
                    id="increment-button"
                    data-input-counter-increment="counter-input"
                    class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none"
                >
                    <svg
                        class="w-2.5 h-2.5 text-slate-950 pointer-events-none"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 18 18"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 1v16M1 9h16"
                        ></path>
                    </svg>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn GridSizeControl(
    grid_rows_num: Signal<u16>,
    #[prop(into)] grid_size_handler: Callback<Operation>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col ml-6">
            <label
                for="rows-input"
                class="block mb-1 text-xs font-medium text-slate-950 select-none"
            >
                "Rows"
            </label>
            <div class="relative flex items-center">
                <button
                    on:click=move |_| {
                        grid_size_handler(Operation::Dec);
                    }

                    type="button"
                    id="decrement-button"
                    data-input-counter-decrement="counter-input"
                    class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none"
                    style=(
                        "opacity",
                        move || { if grid_rows_num.get() == GRID_ROWS_MIN { "0.6" } else { "1" } },
                    )

                    disabled=move || { grid_rows_num.get() == 0 }
                >
                    <svg
                        class="w-2.5 h-2.5 text-slate-950 pointer-events-none"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 18 2"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1h16"
                        ></path>
                    </svg>
                </button>
                <input
                    type="text"
                    id="rows-input"
                    data-input-counter
                    class="flex-shrink-0 text-slate-950 border-0 bg-transparent text-xs font-normal focus:outline-none focus:ring-0 max-w-[2.5rem] text-center select-none"
                    placeholder=""
                    prop:value=grid_rows_num
                    disabled=true
                />

                <button
                    on:click=move |_| {
                        grid_size_handler(Operation::Inc);
                    }

                    type="button"
                    id="increment-button"
                    data-input-counter-increment="counter-input"
                    class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none"
                    style=(
                        "opacity",
                        move || { if grid_rows_num.get() == GRID_ROWS_MAX { "0.6" } else { "1" } },
                    )
                >
                    <svg
                        class="w-2.5 h-2.5 text-slate-950 pointer-events-none"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 18 18"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 1v16M1 9h16"
                        ></path>
                    </svg>
                </button>
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
        <div class="w-6 h-6 flex mr-4 cursor-pointer"
            on:click=move |_| { set_random_playback.update(|val| *val = !*val)}
        >
            <ArrowsRightLeft
                class=move || format!("cursor-pointer{}", if random_playback.get() {" stroke-blue-500"} else {" stroke-slate-950"})
            />
        </div>
    }
}
