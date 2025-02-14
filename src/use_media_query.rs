#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::use_event_listener;
use crate::utils::CloneableFnMutWithArg;
use cfg_if::cfg_if;
use leptos::ev::change;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Reactive [Media Query](https://developer.mozilla.org/en-US/docs/Web/CSS/Media_Queries/Testing_media_queries).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_media_query)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_media_query;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// #
/// let is_large_screen = use_media_query(cx, "(min-width: 1024px)");
///
/// let is_dark_preferred = use_media_query(cx, "(prefers-color-scheme: dark)");
/// #
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this functions returns a Signal that is always `false`.
///
/// ## See also
///
/// * [`use_preferred_dark`]
/// * [`use_preferred_contrast`]
pub fn use_media_query(cx: Scope, query: impl Into<MaybeSignal<String>>) -> Signal<bool> {
    let query = query.into();

    let (matches, set_matches) = create_signal(cx, false);

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let media_query: Rc<RefCell<Option<web_sys::MediaQueryList>>> = Rc::new(RefCell::new(None));
        let remove_listener: RemoveListener = Rc::new(RefCell::new(None));

        let listener: Rc<RefCell<Box<dyn CloneableFnMutWithArg<web_sys::Event>>>> =
            Rc::new(RefCell::new(Box::new(|_| {})));

        let cleanup = {
            let remove_listener = Rc::clone(&remove_listener);

            move || {
                if let Some(remove_listener) = remove_listener.take().as_ref() {
                    remove_listener();
                }
            }
        };

        let update = {
            let cleanup = cleanup.clone();
            let listener = Rc::clone(&listener);

            move || {
                cleanup();

                let mut media_query = media_query.borrow_mut();
                *media_query = window().match_media(&query.get()).unwrap_or(None);

                if let Some(media_query) = media_query.as_ref() {
                    set_matches.set(media_query.matches());

                    remove_listener.replace(Some(Box::new(use_event_listener(
                        cx,
                        media_query.clone(),
                        change,
                        listener.borrow().clone(),
                    ))));
                } else {
                    set_matches.set(false);
                }
            }
        };

        {
            let update = update.clone();
            listener
                .replace(Box::new(move |_| update()) as Box<dyn CloneableFnMutWithArg<web_sys::Event>>);
        }

        create_effect(cx, move |_| update());

        on_cleanup(cx, cleanup);
    }}

    matches.into()
}

type RemoveListener = Rc<RefCell<Option<Box<dyn Fn()>>>>;
