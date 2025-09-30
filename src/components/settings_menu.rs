use crate::shared::{Operation, GRID_ROWS_MAX, GRID_ROWS_MIN};
use leptos::{prelude::*, *};
use leptos_heroicons::size_24::outline::{Bars2, CalendarDays, Folder, XMark};
use leptos_use::on_click_outside;
use web_sys::HtmlInputElement;

#[component]
pub fn SettingsMenu(
    gap_duration: ReadSignal<u64>,
    set_gap_duration: WriteSignal<u64>,
    grid_rows_num: Signal<u16>,
    #[prop(into)] grid_size_handler: Callback<Operation>,
    set_presets_visible: WriteSignal<bool>,
    set_schedule_visible: WriteSignal<bool>,
    #[prop(into)] erase_grid_handler: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let (open, set_open) = signal(false);

    let menu_ref = NodeRef::new();
    let _ = on_click_outside(menu_ref, move |_| set_open.set(false));

    let menu_base_class =
        "absolute -bottom-[270px] right-4 w-40 h-content rounded-lg bg-white z-10 flex flex-col items-start gap-2 px-4 py-2 cursor-default shadow";

    view! {
        <div class="relative flex justify-end">
            <button
                class="p-2 cursor-pointer"
                on:click=move |_| set_open.update(|val| { *val = !*val })
            >
                <Show
                    when=open
                    fallback=move || {
                        view! { <Bars2 class="w-8 h-8 stroke-white" /> }
                    }
                >
                    <XMark class="w-8 h-8 stroke-white" />

                </Show>
            </button>
            <div
                class=move || {
                    format!("{menu_base_class} {}", if open.get() { "" } else { "hidden" })
                }
                node_ref=menu_ref
            >
                <PlaybackGapDuration gap_duration set_gap_duration />
                <GridSizeControl grid_rows_num grid_size_handler />
                <EraseGridButton erase_grid_handler set_open />
                <div class="border-b-[1px] border-slate-200 w-full -mb-2"></div>
                <PresetsButton set_presets_visible set_open />
                <ScheduleButton set_schedule_visible set_open />
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
    let input_container_class =
        "relative flex items-center border-[1px] border-slate-200 rounded-full p-1";
    let inc_dec_button_class = "flex-shrink-0 bg-slate-100 hover:bg-slate-200 inline-flex items-center justify-center border border-slate-300 rounded-full h-8 w-8 focus:ring-gray-100 focus:ring-2 focus:outline-none";

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
                class="block mb-1 text-xs font-medium text-slate-950 text-left select-none"
            >
                "Silent gap (sec)"
            </label>
            <div class=input_container_class>
                <button
                    on:click=move |_| { inc_dec_handler(Operation::Dec) }
                    type="button"
                    id="decrement-button"
                    data-input-counter-decrement="counter-input"
                    class=inc_dec_button_class
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
                    class="flex-shrink-0 text-slate-950 border-0 bg-transparent text-sm font-normal focus:outline-none focus:ring-0 max-w-[2.5rem] text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                    placeholder=""
                    inputmode="numeric"
                    prop:value=move || { gap_duration.get() as f64 / 1000_f64 }
                    on:input=input_handler
                />

                <button
                    on:click=move |_| { inc_dec_handler(Operation::Inc) }
                    type="button"
                    id="increment-button"
                    data-input-counter-increment="counter-input"
                    class=inc_dec_button_class
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
    let container_class = "flex flex-col";
    let input_container_class =
        "relative flex items-center border-[1px] border-slate-200 rounded-full p-1";
    let inc_dec_button_class = "flex-shrink-0 bg-slate-100 hover:bg-slate-200 inline-flex items-center justify-center border border-slate-300 rounded-full h-8 w-8 focus:ring-gray-100 focus:ring-2 focus:outline-none";

    view! {
        <div class=container_class>
            <label
                for="rows-input"
                class="block mb-1 text-xs font-medium text-slate-950 text-left select-none"
            >
                "Rows"
            </label>
            <div class=input_container_class>
                <button
                    on:click=move |_| {
                        grid_size_handler.run(Operation::Dec);
                    }

                    type="button"
                    id="decrement-button"
                    data-input-counter-decrement="counter-input"
                    class=inc_dec_button_class
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
                    class="flex-shrink-0 text-slate-950 border-0 bg-transparent text-sm font-normal focus:outline-none focus:ring-0 max-w-[2.5rem] text-center select-none"
                    placeholder=""
                    prop:value=grid_rows_num
                    disabled=true
                />

                <button
                    on:click=move |_| {
                        grid_size_handler.run(Operation::Inc);
                    }

                    type="button"
                    id="increment-button"
                    data-input-counter-increment="counter-input"
                    class=inc_dec_button_class
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
pub fn EraseGridButton(
    erase_grid_handler: Callback<ev::MouseEvent>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            class="flex items-center select-none p-2 -ml-2 hover:bg-red-50 hover:text-red-500 rounded w-full text-sm text-slate-900 font-medium"
            on:click=move |e| {
                erase_grid_handler.run(e);
                set_open(false);
            }
        >
            Erase grid
        </button>
    }
}

#[component]
pub fn PresetsButton(
    set_presets_visible: WriteSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    let container_class =
        "flex items-center cursor-pointer select-none p-2 -ml-2 hover:bg-slate-100 rounded w-full";

    view! {
        <button
            class=container_class
            on:click=move |_| {
                set_presets_visible.update(|val| *val = !*val);
                set_open.set(false);
            }
        >
            <Folder class="w-5 h-5 stroke-slate-900 mr-2 stroke-2" />
            <span class="text-sm text-slate-900 font-medium">Presets</span>
        </button>
    }
}

#[component]
pub fn ScheduleButton(
    set_schedule_visible: WriteSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    let container_class =
        "flex items-center cursor-pointer select-none p-2 -ml-2 -mt-2 hover:bg-slate-100 rounded w-full";

    view! {
        <button
            class=container_class
            on:click=move |_| {
                set_schedule_visible.update(|val| *val = !*val);
                set_open.set(false);
            }
        >
            <CalendarDays class="w-5 h-5 mr-2 stroke-slate-900 stroke-2" />
            <span class="text-sm text-slate-900 font-medium">Schedule</span>
        </button>
    }
}
