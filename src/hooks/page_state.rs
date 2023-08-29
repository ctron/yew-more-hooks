//! Hooks for working with the page state
//!
//! ## Example
//!
//! ```rust
//! use yew::prelude::*;
//!
//! #[function_component(Example)]
//! fn example() -> Html {
//!   #[derive(Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
//!   struct PageState {}
//!
//!   let page_state = use_page_state(PageState::default);
//!
//!   // make use of `page_state` here to initialize others
//!
//!   use_page_state_update(page_state, PageState {
//!     // new page state, will be stored when changed
//!   });
//!
//!   html!()
//! }
//! ```
use gloo_utils::format::JsValueSerdeExt;
use std::ops::Deref;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UsePageState<S>
where
    for<'de> S: Clone + PartialEq + serde::Serialize + serde::Deserialize<'de> + 'static,
{
    state: UseStateHandle<S>,
}

impl<S> Deref for UsePageState<S>
where
    for<'de> S: Clone + PartialEq + serde::Serialize + serde::Deserialize<'de> + 'static,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &*self.state
    }
}

impl<S> UsePageState<S>
where
    for<'de> S: Clone + PartialEq + serde::Serialize + serde::Deserialize<'de> + 'static,
{
    /// update the state by receiving the current and returning a new one
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(S) -> S,
    {
        self.state.set(f((*self.state).clone()))
    }

    /// modify the state by modifying an instance
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(&mut S),
    {
        self.update(|mut state| {
            f(&mut state);
            state
        });
    }

    /// replace the current page state with the new one
    pub fn set(&self, state: S) {
        self.state.set(state);
    }
}

/// A hook which keeps track of a state of the current page.
///
/// The hook initializes its value either from deserializing the current page state, or falling back
/// to using the init function.
///
/// Whenever the state handler returned by the hook changes, it will store a new state for the
/// current page.
#[hook]
pub fn use_page_state<S, F>(init: F) -> UsePageState<S>
where
    for<'de> S: Clone + PartialEq + serde::Serialize + serde::Deserialize<'de> + 'static,
    F: FnOnce() -> S,
{
    let state = use_state_eq(|| {
        let history = gloo_utils::history();
        history
            .state()
            .map_err(|_| {})
            .and_then(|state| state.into_serde().map_err(|_| ()))
            .unwrap_or_else(|_| init())
    });

    use_effect_with_deps(
        |state| {
            let history = gloo_utils::history();
            if let Ok(state) = JsValue::from_serde(&state) {
                let _ = history.replace_state(&state, "");
            }
        },
        (*state).clone(),
    );

    UsePageState { state }
}

/// A hook to update the page state whenever it changes.
///
/// This is a convenience wrapper using [`use_effect_with_deps`] to update the [`UsePageState`]
/// whenever it changes.
#[hook]
pub fn use_page_state_update<S>(page_state: UsePageState<S>, new_page_state: S)
where
    for<'de> S: Clone + PartialEq + serde::Serialize + serde::Deserialize<'de> + 'static,
{
    use_effect_with_deps(
        |(page_state, new_page_state)| {
            page_state.set(new_page_state.clone());
        },
        (page_state.clone(), new_page_state),
    );
}
