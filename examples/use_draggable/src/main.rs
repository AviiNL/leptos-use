use leptos::html::Div;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref::<Div>(cx);

    let inner_width = window().inner_width().unwrap().as_f64().unwrap();

    let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
        cx,
        el,
        UseDraggableOptions::default()
            .initial_value(Position {
                x: inner_width / 2.2,
                y: 80.0,
            })
            .prevent_default(true),
    );

    view! { cx,
        <p class="italic op50 text-center">
            Check the floating box
        </p>
        <div
            node_ref=el
            class="px-4 py-2 border border-gray-400/30 rounded shadow hover:shadow-lg fixed bg-[--bg] select-none cursor-move z-24"
            style=move || format!("touch-action: none; {}", style())
        >
            "👋 Drag me!"
            <div class="text-sm opacity-50">
                I am at { move || x().round() }, { move || y().round() }
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
