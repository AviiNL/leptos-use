use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_window_focus, watch};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let start_message = "💡 Click somewhere outside of the document to unfocus.";

    let (message, set_message) = create_signal(cx, start_message);

    let focused = use_window_focus(cx);

    let _ = watch(cx, focused, move |focused, _, _| {
        if *focused {
            set_message(start_message);
        } else {
            set_message("ℹ Tab is unfocused")
        }
    });

    view! { cx,
        <div>{ message }</div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
