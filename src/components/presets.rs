use std::collections::HashSet;

use crate::{
    components::button::Button,
    shared::{Preset, Sample},
};
use chrono::Utc;
use ev::KeyboardEvent;
use html::Input;
use leptos::*;
use leptos_heroicons::size_24::outline::{ArrowsRightLeft, CheckCircle, Trash};

#[component]
pub fn Presets(
    presets_visible: ReadSignal<bool>,
    set_presets_visible: WriteSignal<bool>,
    #[prop(into)] save_preset_handler: Callback<String>,
    presets: ReadSignal<Vec<Preset>>,
    #[prop(into)] delete_preset_handler: Callback<String>,
    #[prop(into)] load_preset_handler: Callback<Preset>,
) -> impl IntoView {
    let (show_prompt, set_show_prompt) = create_signal(false);

    let summon_prompt = Callback::new(move |_| {
        set_show_prompt.set(true);
    });

    let save_preset_local_handler = Callback::new(move |preset_name| {
        set_show_prompt(false);
        save_preset_handler(preset_name);
    });

    let container_class =
        "absolute top-0 right-0 bottom-auto left-0 min-h-screen w-screen bg-white p-2";

    let button_base_class = "group rounded p-2 width-fit flex";
    let button_base_icon_class = "w-4 h-4 mr-1 stroke-slate-950 stroke-2 select-none";
    let button_base_text_class = "text-xs text-slate-950 font-medium select-none";

    view! {
        <div
            class=container_class
            style:display=move || { if presets_visible.get() { "block" } else { "none" } }
        >
            <div>
                <For
                    each=move || presets.get()
                    key=move |preset| preset.id.clone()
                    children=move |preset: Preset| {
                        let total_cells_num = preset.grid_data.len();
                        let filled_cells: Vec<&Option<Sample>> = preset
                            .grid_data
                            .iter()
                            .filter(|cell| cell.is_some())
                            .collect();
                        let filled_cells_num = filled_cells.len();
                        let category_emojis_hs = filled_cells
                            .iter()
                            .map(|cell| { cell.as_ref().unwrap().category.get_emoji() })
                            .collect::<HashSet<char>>();
                        let mut category_emojis_vec = category_emojis_hs
                            .iter()
                            .collect::<Vec<&char>>();
                        category_emojis_vec.sort();
                        let category_emojis = category_emojis_vec.into_iter().collect::<String>();
                        let preset_id = preset.id.clone();
                        view! {
                            <div class="px-4 py-2 mb-2 border-2 border-slate-200 rounded-lg flex items-center cursor-pointer hover:border-slate-300">
                                <div class=" mr-1 sm:mr-2 flex-1">
                                    <h1 class="select-none text-sm max-w-[80px] sm:max-w-[200px] whitespace-nowrap overflow-hidden text-ellipsis font-semibold">
                                        {preset.name.clone()}
                                    </h1>
                                </div>
                                <div class="mr-1 sm:mr-2 flex-1 select-none">
                                    <div class="text-xs select-none">
                                        <span>{filled_cells_num}/{total_cells_num}</span>
                                        " | "
                                        <span>{format!("{:.2}s", preset.gap_duration / 1000)}</span>
                                        " | "
                                        <span>{category_emojis}</span>
                                        {if preset.random_playback { " | " } else { "" }}
                                        <span>
                                            {if preset.random_playback {
                                                view! { <ArrowsRightLeft class="inline w-4 h-4" /> }
                                            } else {
                                                view! { "" }.into_view()
                                            }}
                                        </span>
                                    </div>
                                </div>
                                <div class="text-sm flex-1 flex justify-end">
                                    <button
                                        class=format!("{button_base_class} hover:bg-blue-50")
                                        on:click=move |_| {
                                            load_preset_handler(preset.clone());
                                        }
                                    >
                                        <CheckCircle class=format!(
                                            "{button_base_icon_class} group-hover:stroke-blue-600",
                                        ) />
                                        <span class=format!(
                                            "{button_base_text_class} group-hover:text-blue-600",
                                        )>Apply preset</span>
                                    </button>
                                    <button
                                        class=format!("{button_base_class} hover:bg-red-50")
                                        on:click=move |_| {
                                            delete_preset_handler(preset_id.clone());
                                        }
                                    >
                                        <Trash class=format!(
                                            "{button_base_icon_class} group-hover:stroke-red-600",
                                        ) />
                                        <span class=format!(
                                            "{button_base_text_class} group-hover:text-red-600",
                                        )>Delete</span>
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            <ControlPanel set_presets_visible save_preset_handler=summon_prompt />

            <Prompt
                title=String::from("Enter preset name")
                show=show_prompt
                on_click=save_preset_local_handler
            />
        </div>
    }
}

#[component]
pub fn ControlPanel(
    set_presets_visible: WriteSignal<bool>,
    save_preset_handler: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let container = "fixed bottom-[4%] w-screen h-[56px]";
    let container_inner = "w-60 h-[100%] mx-auto flex items-center justify-center";

    view! {
        <div class=container>
            <div class=container_inner>
                <Button class="mr-4" on:click=save_preset_handler>
                    Save preset
                </Button>
                <Button on:click=move |_| {
                    set_presets_visible.update(|val| { *val = !*val })
                }>Close</Button>
            </div>
        </div>
    }
}

#[component]
pub fn Prompt(title: String, on_click: Callback<String>, show: ReadSignal<bool>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();

    let backdrop = "fixed top-0 right-0 bottom-0 left-0 bg-slate-200/20 backdrop-blur";
    let prompt_container =
        "absolute flex flex-col justify-center items-center mx-auto top-[30%] right-0 left-0 w-fit h-40 bg-white rounded-lg shadow-lg px-8 py-2";

    create_effect(move |_| {
        if show.get() {
            if let Some(input_ref) = input_ref.get() {
                let _ = input_ref.on_mount(|elem| {
                    elem.focus().unwrap();
                });
            };
        }
    });

    let on_enter_press = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            let input = input_ref.get().unwrap();
            let val = input.value();
            input.set_value("");
            on_click(val);
        }
    };

    let click_button_handler = move |_| {
        let input = input_ref.get().unwrap();
        let val = input.value();
        input.set_value("");
        on_click(val);
    };

    view! {
        <div class=move || { format!("{backdrop}{}", if show.get() { "" } else { " hidden" }) }>
            <div class=prompt_container>
                <h1 class="text-center m-0 mb-4 text-sm font-semibold">{title}</h1>
                <input
                    class="p-2 border-2 rounded-lg mb-6"
                    on:keydown=on_enter_press
                    _ref=input_ref
                    placeholder=format!("Preset {}", Utc::now().format("%Y.%m.%d %H:%M"))
                    maxlength=40
                />
                <Button class="shadow-none" on:click=click_button_handler>
                    Save preset
                </Button>
            </div>
        </div>
    }
}
