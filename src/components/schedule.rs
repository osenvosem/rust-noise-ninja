use std::collections::HashSet;

use crate::shared::{PlannedSchedule, RecurringSchedule, RecurringScheduleOverlap, ScheduleType};
use crate::{components::button::Button, shared::Preset};
use chrono::{Duration, Local, NaiveDateTime, NaiveTime, Weekday};
use ev::{KeyboardEvent, MouseEvent};
use html::{Div, Input, Select};
use leptos::*;
use leptos_heroicons::size_24::outline::Trash;
use web_sys::{HtmlButtonElement, HtmlInputElement};

#[component]
pub fn Schedule(
    schedule_visible: ReadSignal<bool>,
    set_schedule_visible: WriteSignal<bool>,
    #[prop(into)] save_planned_schedule: Callback<PlannedSchedule>,
    #[prop(into)] delete_planned_schedule: Callback<String>,
    #[prop(into)] save_recurring_schedule: Callback<RecurringSchedule>,
    #[prop(into)] delete_recurring_schedule: Callback<String>,
    planned_schedules: ReadSignal<Vec<PlannedSchedule>>,
    recurring_schedules: ReadSignal<Vec<RecurringSchedule>>,
    schedule_type: ReadSignal<ScheduleType>,
    set_schedule_type: WriteSignal<ScheduleType>,
    presets: ReadSignal<Vec<Preset>>,
) -> impl IntoView {
    let (show_planned_schedule_prompt, set_show_planned_schedule_prompt) = create_signal(false);
    let (show_recurring_schedule_prompt, set_show_recurring_schedule_prompt) = create_signal(false);

    let summon_prompt = Callback::new(move |_| {
        if schedule_type.get() == ScheduleType::Planned {
            set_show_planned_schedule_prompt.set(true);
        } else {
            set_show_recurring_schedule_prompt.set(true);
        }
    });

    let save_planned_schedule_local_handler =
        Callback::new(move |planned_schedule: PlannedSchedule| {
            set_show_planned_schedule_prompt.set(false);
            save_planned_schedule(planned_schedule);
        });

    let save_recurring_schedule_local_handler =
        Callback::new(move |recurring_schedule: RecurringSchedule| {
            set_show_recurring_schedule_prompt.set(false);
            save_recurring_schedule(recurring_schedule);
        });

    let container_class =
        "absolute top-0 right-0 bottom-auto left-0 min-h-screen w-screen bg-white p-2";

    view! {
        <div
            class=container_class
            style:display=move || { if schedule_visible.get() { "block" } else { "none" } }
        >
            <Tabs schedule_type set_schedule_type />
            <Show when=move || schedule_type.get() == ScheduleType::Planned>
                <For
                    each=move || planned_schedules.get()
                    key=move |s| s.id.clone()
                    children=move |schedule| {
                        view! {
                            <PlannedScheduleItem schedule delete_schedule=delete_planned_schedule />
                        }
                    }
                />
            </Show>

            <Show when=move || schedule_type.get() == ScheduleType::Reccuring>
                <For
                    each=move || recurring_schedules.get()
                    key=move |s| s.id.clone()
                    children=move |schedule| {
                        view! {
                            <RecurringScheduleItem
                                schedule
                                delete_schedule=delete_recurring_schedule
                            />
                        }
                    }
                />
            </Show>

            <ControlPanel set_schedule_visible add_period=summon_prompt />

            <Show when=move || show_planned_schedule_prompt.get()>
                <PlannedSchedulePrompt
                    title=String::from("New schedule period")
                    show=show_planned_schedule_prompt
                    set_show=set_show_planned_schedule_prompt
                    on_click=save_planned_schedule_local_handler
                    presets
                    planned_schedules
                />
            </Show>
            <Show when=move || show_recurring_schedule_prompt.get()>
                <RecurringSchedulePrompt
                    title=String::from("New schedule period")
                    show=show_recurring_schedule_prompt
                    set_show=set_show_recurring_schedule_prompt
                    on_click=save_recurring_schedule_local_handler
                    presets
                    recurring_schedules
                />
            </Show>
        </div>
    }
}

#[component]
pub fn ControlPanel(
    set_schedule_visible: WriteSignal<bool>,
    add_period: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let container = "fixed bottom-[4%] w-screen h-[56px]";
    let container_inner = "w-60 h-[100%] mx-auto flex items-center justify-center";

    view! {
        <div class=container>
            <div class=container_inner>
                <Button class="mr-4" on:click=add_period>
                    Add period
                </Button>
                <Button on:click=move |_| {
                    set_schedule_visible.update(|val| { *val = !*val })
                }>Close</Button>
            </div>
        </div>
    }
}

#[component]
pub fn PlannedSchedulePrompt(
    title: String,
    on_click: Callback<PlannedSchedule>,
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    presets: ReadSignal<Vec<Preset>>,
    planned_schedules: ReadSignal<Vec<PlannedSchedule>>,
) -> impl IntoView {
    let backdrop = "fixed top-0 right-0 bottom-0 left-0 bg-slate-200/20 backdrop-blur";
    let prompt_container =
        "absolute flex flex-col justify-center items-center mx-auto top-[20%] right-0 left-0 w-80 bg-white rounded-lg shadow-lg px-6 py-2";

    let start_input_ref = create_node_ref::<Input>();
    let end_input_ref = create_node_ref::<Input>();
    let preset_input_ref = create_node_ref::<Select>();

    let datetime_format = "%Y-%m-%dT%H:%M";

    let dt_start_min = {
        let s = planned_schedules.get();
        if !s.is_empty() {
            (s.last().unwrap().end + Duration::minutes(1))
                .format(datetime_format)
                .to_string()
        } else {
            Local::now().format(datetime_format).to_string()
        }
    };

    let (dt_start, set_dt_start) = create_signal(dt_start_min.clone());
    let (dt_end, set_dt_end) = create_signal(String::new());

    let dt_end_min = move || {
        (NaiveDateTime::parse_from_str(dt_start.get().as_str(), datetime_format).unwrap()
            + Duration::minutes(1))
        .format(datetime_format)
        .to_string()
    };

    let should_be_disabled = Signal::derive(move || presets.get().is_empty());

    create_effect(move |_| {
        set_dt_end.set(dt_end_min());
    });

    create_effect(move |_| {
        let start =
            NaiveDateTime::parse_from_str(dt_start.get().as_str(), datetime_format).unwrap();
        let end = NaiveDateTime::parse_from_str(dt_end.get().as_str(), datetime_format).unwrap();

        if start >= end {
            set_dt_end.set(dt_end_min());
        };
    });

    let click_save_button_handler = move |_| {
        let start_val = start_input_ref.get().unwrap().value();
        let end_val = end_input_ref.get().unwrap().value();
        let preset_val = preset_input_ref.get().unwrap().value();

        let start = NaiveDateTime::parse_from_str(start_val.as_str(), datetime_format).unwrap();
        let end = NaiveDateTime::parse_from_str(end_val.as_str(), datetime_format).unwrap();
        let id = Local::now().timestamp().to_string();

        let preset = presets
            .get()
            .into_iter()
            .find(|p| preset_val == p.id)
            .unwrap();

        let schedule = PlannedSchedule {
            id,
            start,
            end,
            preset,
        };

        on_click(schedule);
    };

    let click_cancel_button_handler = move |_: MouseEvent| {
        set_show.set(false);
    };

    let on_blur_dt_start_handler = move |e: ev::FocusEvent| {
        let val = event_target_value(&e);

        set_dt_start.set(val);
    };

    let on_blur_dt_end_handler = move |e: ev::FocusEvent| {
        let val = event_target_value(&e);

        set_dt_end.set(val);
    };

    let on_enter_handler = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            let elem = event_target::<HtmlInputElement>(&e);
            if let Some(id) = elem.get_attribute("id") {
                if id == "input-start" {
                    start_input_ref.get().unwrap().blur().unwrap();
                } else if id == "input-end" {
                    end_input_ref.get().unwrap().blur().unwrap();
                };
            }
        };
    };

    view! {
        <div class=move || { format!("{backdrop}{}", if show.get() { "" } else { " hidden" }) }>
            <div class=prompt_container>
                <h1 class="text-center m-0 mb-4 text-sm font-semibold">{title}</h1>
                <fieldset class="flex flex-col mb-2 w-full">
                    <label for="input-start" class="text-xs ml-2">
                        Start
                    </label>
                    <input
                        id="input-start"
                        type="datetime-local"
                        min=dt_start_min
                        prop:value=dt_start
                        class="p-1 border-2 rounded-lg mb-1 w-full"
                        _ref=start_input_ref
                        on:blur=on_blur_dt_start_handler
                        on:keydown=on_enter_handler
                    />
                </fieldset>
                <fieldset class="flex flex-col mb-2 w-full">
                    <label for="input-end" class="text-xs ml-2">
                        End
                    </label>
                    <input
                        id="input-end"
                        type="datetime-local"
                        min=dt_end_min
                        prop:value=dt_end
                        class="p-1 border-2 rounded-lg mb-1 w-full"
                        _ref=end_input_ref
                        on:blur=on_blur_dt_end_handler
                        on:keydown=on_enter_handler
                    />
                </fieldset>
                <fieldset class="flex flex-col w-full">
                    <label for="input-end" class="text-xs ml-2">
                        Preset
                    </label>
                    <div class="relative before:block before:absolute before:right-2 before:top-[40%] before:w-0 before:h-0 before:content-[''] before:z-10 before:border-l-[6px] before:border-l-transparent before:border-t-[6px] before:border-t-gray-300 before:border-r-[6px] before:border-r-transparent before:pointer-events-none">
                        <select
                            id="input-end"
                            class="block relative appearance-none p-1 border-2 rounded-lg mb-1 w-full "
                            _ref=preset_input_ref
                            disabled=should_be_disabled
                        >
                            {move || {
                                presets
                                    .get()
                                    .iter()
                                    .map(|p| {
                                        view! { <option value=&p.id>{&p.name}</option> }
                                    })
                                    .collect_view()
                            }}
                        </select>
                    </div>
                </fieldset>
                <Show when=move || presets.get().is_empty()>
                    <div class="flex flex-col flex-start p-2 rounded bg-red-50 mt-1">
                        <h4 class="text-xs text-red-500 font-semibold">
                            For creating a schedule you must create a preset
                        </h4>
                    </div>
                </Show>
                <div class="flex flex-row mt-4">
                    <Button
                        class="shadow-none mb-2 mr-2"
                        on:click=click_save_button_handler
                        disabled=should_be_disabled
                    >
                        Save schedule
                    </Button>
                    <Button class="shadow-none mb-2" on:click=click_cancel_button_handler>
                        Cancel
                    </Button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RecurringSchedulePrompt(
    title: String,
    on_click: Callback<RecurringSchedule>,
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    presets: ReadSignal<Vec<Preset>>,
    recurring_schedules: ReadSignal<Vec<RecurringSchedule>>,
) -> impl IntoView {
    let backdrop = "fixed top-0 right-0 bottom-0 left-0 bg-slate-200/20 backdrop-blur";
    let prompt_container =
        "absolute flex flex-col justify-center items-center mx-auto top-[10%] right-0 left-0 w-80 bg-white rounded-lg shadow-lg px-6 py-2 h-auto";

    let time_format = "%H:%M:%S";
    let weekdays_default = vec![
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];

    let weekdays_elem_ref = create_node_ref::<Div>();
    let start_input_ref = create_node_ref::<Input>();
    let end_input_ref = create_node_ref::<Input>();
    let preset_input_ref = create_node_ref::<Select>();
    let container_ref = create_node_ref::<Div>();

    let (t_start, set_t_start) = create_signal(String::from("12:00"));
    let (t_end, set_t_end) = create_signal(String::from("12:01"));
    let (weekdays, set_weekdays) = create_signal::<Vec<Weekday>>(weekdays_default);
    let (overlapping_error, set_overlapping_error) =
        create_signal::<Option<Vec<RecurringScheduleOverlap>>>(None);

    let should_be_disabled =
        Signal::derive(move || presets.get().is_empty() || overlapping_error.get().is_some());

    let t_end_min = move || {
        let mut start = t_start.get();
        start.push_str(":00");
        (NaiveTime::parse_from_str(start.as_str(), time_format).unwrap() + Duration::minutes(1))
            .format("%H:%M")
            .to_string()
    };

    // NOTE: this hack allows for avoiding visual gap in modal window in Safari
    create_effect(move |_| {
        if overlapping_error.get().is_some() {
            let elem = container_ref.get().unwrap();
            let height = elem.offset_height();
            let _ = elem.style("height", format!("{}px", height + 1));
        };
    });

    create_effect(move |_| {
        let mut start = t_start.get();
        start.push_str(":00");
        let mut end = t_end.get();
        end.push_str(":00");

        let start_nt = NaiveTime::parse_from_str(start.as_str(), time_format).unwrap();
        let end_nt = NaiveTime::parse_from_str(end.as_str(), time_format).unwrap();

        if start_nt >= end_nt {
            set_t_end.set(t_end_min());
        }
    });

    let click_save_button_handler = move |_: MouseEvent| {
        let mut start_val = start_input_ref.get().unwrap().value();
        let mut end_val = end_input_ref.get().unwrap().value();
        let preset_val = preset_input_ref.get().unwrap().value();

        start_val.push_str(":00");
        end_val.push_str(":00");

        let start = NaiveTime::parse_from_str(start_val.as_str(), time_format).unwrap();
        let end = NaiveTime::parse_from_str(end_val.as_str(), time_format).unwrap();
        let id = Local::now().timestamp().to_string();

        let preset = presets
            .get()
            .into_iter()
            .find(|p| preset_val == p.id)
            .unwrap();
        let weekdays = weekdays.get();

        let schedules = recurring_schedules.get();
        let conflicting_periods = schedules
            .iter()
            .filter_map(|s| {
                let a_hs: HashSet<Weekday> = HashSet::from_iter(s.weekdays.iter().cloned());
                let b_hs = HashSet::from_iter(weekdays.iter().cloned());

                let weekday_intersections =
                    a_hs.intersection(&b_hs).copied().collect::<Vec<Weekday>>();

                if weekday_intersections.is_empty() {
                    None
                } else {
                    let case_one = start >= s.start && start <= s.end;
                    let case_two = end >= s.start && end <= s.end;
                    let case_three = start <= s.start && end >= s.end;

                    if case_one || case_two || case_three {
                        Some(RecurringScheduleOverlap {
                            weekdays: weekday_intersections,
                            start: s.start,
                            end: s.end,
                        })
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<RecurringScheduleOverlap>>();

        if conflicting_periods.is_empty() {
            let schedule = RecurringSchedule {
                id,
                weekdays,
                start,
                end,
                preset,
            };

            set_overlapping_error.set(None);

            on_click(schedule);
        } else {
            set_overlapping_error.set(Some(conflicting_periods));
        }
    };

    let click_cancel_button_handler = move |_: MouseEvent| {
        set_show.set(false);
    };

    let on_blur_handler = move |e: ev::FocusEvent| {
        let target = event_target::<HtmlInputElement>(&e);

        if target.id() == "input-start" {
            set_t_start.set(target.value());
        } else if target.id() == "input-end" {
            set_t_end.set(target.value());
        }

        set_overlapping_error(None);
    };

    let on_enter_handler = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            let elem = event_target::<HtmlInputElement>(&e);
            if let Some(id) = elem.get_attribute("id") {
                if id == "input-start" {
                    start_input_ref.get().unwrap().blur().unwrap();
                } else if id == "input-end" {
                    end_input_ref.get().unwrap().blur().unwrap();
                };
            }
        };
    };

    let weekday_button_class =
        "p-2 cursor-pointer text-xs bg-blue-500 text-white first:rounded-l-lg last:rounded-r-lg";

    let weekday_button_handler = move |e: MouseEvent| {
        let target = event_target::<HtmlButtonElement>(&e);
        if target.tag_name().to_lowercase() == "button" {
            if let Some(node) = target.first_child() {
                let val = node.node_value().unwrap().parse::<Weekday>().unwrap();
                match target.get_attribute("data-selected") {
                    Some(_) => {
                        set_weekdays.update(|wd| {
                            let idx = wd.iter().position(|x| *x == val).unwrap();
                            wd.remove(idx);
                        });
                    }
                    None => {
                        set_weekdays.update(|wd| {
                            wd.push(val);
                        });
                    }
                }
            }

            let cl = target.class_list();
            cl.toggle("bg-blue-500").unwrap();
            cl.toggle("text-white").unwrap();
            target.toggle_attribute("data-selected").unwrap();
        }
    };

    view! {
        <div class=move || { format!("{backdrop}{}", if show.get() { "" } else { " hidden" }) }>
            <div class=prompt_container _ref=container_ref>
                <h1 class="text-center m-0 mb-4 text-sm font-semibold">{title}</h1>
                <fieldset class="flex justify-center mb-2 w-full">
                    <div
                        class="flex mr-1 bg-slate-50 rounded-full"
                        on:click=weekday_button_handler
                        _ref=weekdays_elem_ref
                    >
                        {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                            .iter()
                            .map(|d| {
                                view! {
                                    <button class=weekday_button_class data-selected>
                                        {*d}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </fieldset>
                <fieldset class="flex flex-col mb-2 w-full">
                    <label for="input-start" class="text-xs ml-2">
                        Start
                    </label>
                    <input
                        id="input-start"
                        type="time"
                        class="p-1 border-2 rounded-lg mb-1 w-full"
                        prop:value=t_start
                        on:keydown=on_enter_handler
                        on:blur=on_blur_handler
                        _ref=start_input_ref
                    />
                </fieldset>
                <fieldset class="flex flex-col mb-2 w-full">
                    <label for="input-end" class="text-xs ml-2">
                        End
                    </label>
                    <input
                        id="input-end"
                        type="time"
                        class="p-1 border-2 rounded-lg mb-1 w-full"
                        min=t_end_min
                        prop:value=t_end
                        on:blur=on_blur_handler
                        on:keydown=on_enter_handler
                        _ref=end_input_ref
                    />
                </fieldset>
                <fieldset class="flex flex-col w-full">
                    <label for="input-end" class="text-xs ml-2">
                        Preset
                    </label>
                    <div class="relative before:block before:absolute before:right-2 before:top-[40%] before:w-0 before:h-0 before:content-[''] before:z-10 before:border-l-[6px] before:border-l-transparent before:border-t-[6px] before:border-t-gray-300 before:border-r-[6px] before:border-r-transparent before:pointer-events-none">
                        <select
                            id="input-end"
                            class="block relative appearance-none p-1 border-2 rounded-lg mb-1 w-full "
                            _ref=preset_input_ref
                            disabled=should_be_disabled
                        >
                            {move || {
                                presets
                                    .get()
                                    .iter()
                                    .map(|p| {
                                        view! { <option value=&p.id>{&p.name}</option> }
                                    })
                                    .collect_view()
                            }}
                        </select>
                    </div>
                </fieldset>
                <Show when=move || overlapping_error.get().is_some()>
                    <div class="flex flex-col flex-start p-2 rounded bg-red-50 mt-1">
                        <h4 class="text-xs text-red-500 font-semibold">
                            Your schedule period is conflicting with:
                        </h4>
                        <ul>
                            {overlapping_error
                                .get()
                                .unwrap()
                                .iter()
                                .map(|err| {
                                    let weekdays = weekdays_to_sorted_vec(err.weekdays.clone())
                                        .join(", ");
                                    let content = format!(
                                        "{} {} - {}",
                                        weekdays,
                                        err.start.format("%H:%M"),
                                        err.end.format("%H:%M"),
                                    );
                                    view! { <li class="text-xs text-red-500">{content}</li> }
                                })
                                .collect_view()}
                        </ul>
                    </div>
                </Show>
                <Show when=move || presets.get().is_empty()>
                    <div class="flex flex-col flex-start p-2 rounded bg-red-50 mt-1">
                        <h4 class="text-xs text-red-500 font-semibold">
                            For creating a schedule you must create a preset
                        </h4>
                    </div>
                </Show>
                <div class="flex flex-row mt-4">
                    <Button
                        class="shadow-none mb-2 mr-2"
                        disabled=should_be_disabled
                        on:click=click_save_button_handler
                    >
                        Save schedule
                    </Button>
                    <Button class="shadow-none mb-2" on:click=click_cancel_button_handler>
                        Cancel
                    </Button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PlannedScheduleItem(
    schedule: PlannedSchedule,
    #[prop(into)] delete_schedule: Callback<String>,
) -> impl IntoView {
    let button_base_class = "group rounded p-2 width-fit flex";
    let button_base_icon_class = "w-4 h-4 mr-1 stroke-slate-950 stroke-2 select-none";
    let button_base_text_class = "text-xs text-slate-950 font-medium select-none";

    view! {
        <div class="px-4 py-2 mb-2 border-2 border-slate-200 rounded-lg flex items-center cursor-pointer hover:border-slate-300 select-none">
            <div class="flex flex-col mr-4">
                <div class="text-xs font-semibold">Start</div>
                <div class="text-sm">{format_datetime(&schedule.start)}</div>
            </div>
            <div class="flex flex-col mr-4">
                <div class="text-xs font-semibold">End</div>
                <div class="text-sm">{format_datetime(&schedule.end)}</div>
            </div>
            <div class="flex flex-col">
                <div class="text-xs font-semibold">Preset</div>
                <div class="text-sm">{schedule.preset.name}</div>
            </div>
            <div class="text-sm flex-1 flex justify-end">
                <button
                    class=format!("{button_base_class} hover:bg-red-50")
                    on:click=move |_| {
                        delete_schedule(schedule.id.clone());
                    }
                >
                    <Trash class=format!("{button_base_icon_class} group-hover:stroke-red-600") />
                    <span class=format!(
                        "{button_base_text_class} group-hover:text-red-600",
                    )>Delete</span>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn RecurringScheduleItem(
    schedule: RecurringSchedule,
    #[prop(into)] delete_schedule: Callback<String>,
) -> impl IntoView {
    let button_base_class = "group rounded p-2 width-fit flex";
    let button_base_icon_class = "w-4 h-4 mr-1 stroke-slate-950 stroke-2 select-none";
    let button_base_text_class = "text-xs text-slate-950 font-medium select-none";

    let weekdays_render = weekdays_to_sorted_vec(schedule.weekdays)
        .iter()
        .map(|wd| {
            view! { <div class="mr-1 px-1 border-2 border-slate-200 rounded">{wd}</div> }
        })
        .collect_view();

    view! {
        <div class="px-4 py-2 mb-2 border-2 border-slate-200 rounded-lg flex items-center cursor-pointer hover:border-slate-300 select-none">
            <div class="flex flex-col max-w-[80px] sm:max-w-[200px] mr-4 w-[200px]">
                <div class="text-xs flex flex-wrap">{weekdays_render}</div>
            </div>
            <div class="flex flex-col mr-4">
                <div class="text-xs font-semibold">Start</div>
                <div class="text-sm">{format_time(&schedule.start)}</div>
            </div>
            <div class="flex flex-col mr-4">
                <div class="text-xs font-semibold">End</div>
                <div class="text-sm">{format_time(&schedule.end)}</div>
            </div>
            <div class="flex flex-col">
                <div class="text-xs font-semibold">Preset</div>
                <div class="text-sm max-w-[80px] sm:max-w-[200px] whitespace-nowrap overflow-hidden text-ellipsis">
                    {schedule.preset.name}
                </div>
            </div>
            <div class="text-sm flex-1 flex justify-end">
                <button
                    class=format!("{button_base_class} hover:bg-red-50")
                    on:click=move |_| {
                        delete_schedule(schedule.id.clone());
                    }
                >
                    <Trash class=format!("{button_base_icon_class} group-hover:stroke-red-600") />
                    <span class=format!(
                        "{button_base_text_class} group-hover:text-red-600",
                    )>Delete</span>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Tabs(
    schedule_type: ReadSignal<ScheduleType>,
    set_schedule_type: WriteSignal<ScheduleType>,
) -> impl IntoView {
    let tab_click_handler = move |e: ev::MouseEvent| {
        let elem = event_target::<HtmlButtonElement>(&e);
        if let Some(t) = elem.get_attribute("data-id") {
            if t == "Planned" {
                set_schedule_type.set(ScheduleType::Planned);
            } else {
                set_schedule_type.set(ScheduleType::Reccuring);
            };
        };
    };

    let tab_active_class = "bg-blue-500 text-white font-semibold";

    view! {
        <div
            class="flex justify-center items-center mb-4 cursor-pointer select-none"
            on:click=tab_click_handler
        >
            <button
                class=move || {
                    format!(
                        "rounded-l-full text-xs py-2 px-4 cursor-pointer {}",
                        if schedule_type.get() == ScheduleType::Reccuring {
                            tab_active_class
                        } else {
                            "bg-slate-100"
                        },
                    )
                }
                data-id="Recurring"
            >
                "Recurring"
            </button>
            <button
                class=move || {
                    format!(
                        "rounded-r-full text-xs py-2 px-4 cursor-pointer {}",
                        if schedule_type.get() == ScheduleType::Planned {
                            tab_active_class
                        } else {
                            "bg-slate-100"
                        },
                    )
                }
                data-id="Planned"
            >
                "Planned"
            </button>
        </div>
    }
}

pub fn format_datetime(datetime: &NaiveDateTime) -> String {
    datetime.format("%d.%m %H:%M").to_string()
}

pub fn format_time(datetime: &NaiveTime) -> String {
    datetime.format("%H:%M").to_string()
}

pub fn weekdays_to_sorted_vec(weekdays: Vec<Weekday>) -> Vec<String> {
    let mut sorted = vec![];
    let weekday_strings: Vec<String> = weekdays.iter().map(|wd| wd.to_string()).collect();
    for (a, b) in [
        ("Mon", "M"),
        ("Tue", "Tu"),
        ("Wed", "W"),
        ("Thu", "Th"),
        ("Fri", "F"),
        ("Sat", "Sa"),
        ("Sun", "Su"),
    ] {
        if weekday_strings.contains(&a.to_string()) {
            sorted.push(b.to_string())
        }
    }
    sorted
}
