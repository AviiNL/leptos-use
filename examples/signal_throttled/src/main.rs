use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::signal_throttled;

#[component]
fn Demo(cx: Scope) -> impl IntoView {

    signal_throttled(cx);

    view! { cx,
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
