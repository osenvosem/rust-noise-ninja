#![allow(non_snake_case)]

mod app;
mod components;
mod shared;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
