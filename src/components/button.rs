use leptos::*;

#[component]
pub fn Button(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] hidden: Signal<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let button_class =
        "border-2 bg-white shadow-lg border-slate-200 rounded-lg py-2 px-4 hover:border-slate-600 hover:shadow-md text:slate-950 text-sm select-none w-fit";

    let click_handler = move |e| {
        if let Some(handler) = on_click {
            handler(e);
        };
    };

    view! {
        <button
            class=move || {
                format!(
                    "{button_class} {class} {} {}",
                    if hidden.get() { "hidden" } else { "" },
                    if disabled.get() { "opacity-30" } else { "" },
                )
            }
            disabled=disabled
            on:click=click_handler
        >
            {children()}
        </button>
    }
}
