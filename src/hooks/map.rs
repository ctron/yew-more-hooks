//! Hooks for mapping

use yew::prelude::*;

/// A hook for mapping data, only when it changes.
///
/// This is a combination of `use_memo` and `Option::map`.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_more_hooks::prelude::*;
///
/// #[derive(PartialEq, Properties)]
/// struct Props {
///   onclick: Option<Callback<()>>,
/// }
///
/// #[function_component(Example)]
/// fn example(props: &Props) -> Html {
///   let onclick: Option<Callback<()>> = use_map(props.onclick.clone(), |onclick| onclick.reform(|_| ()));
///   html!()
/// }
/// ```
///
#[hook]
pub fn use_map<T, U, F>(value: Option<T>, f: F) -> Option<U>
where
    T: PartialEq + 'static,
    U: Clone + 'static,
    F: Fn(&T) -> U + 'static,
{
    (*use_memo(value, |value| value.as_ref().map(f))).clone()
}
