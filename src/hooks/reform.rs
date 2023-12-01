//! Hooks for reforming callback

use yew::prelude::*;

/// Reform a callback.
///
/// Just like [`Callback::reform`], but returning a changed instance only when the original
/// callback changes.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_more_hooks::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   // a callback accepting `()`
///   let onclick = use_callback((), |(), ()| {});
///   // reform to accept `MouseEvent`
///   let onclick = use_reform(onclick, |_| ());
///   // use
///   html!(<button {onclick}>{"Click me"}</button>)
/// }
/// ```
#[hook]
pub fn use_reform<IN1, IN2, OUT, F>(callback: Callback<IN1, OUT>, f: F) -> Callback<IN2, OUT>
where
    IN1: 'static,
    IN2: 'static,
    OUT: 'static,
    F: Fn(IN2) -> IN1 + 'static,
{
    (*use_memo(callback, |callback| callback.reform(f))).clone()
}
