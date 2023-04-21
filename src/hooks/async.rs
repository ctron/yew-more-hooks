// NOTE: This is based on the use_async implementation from https://github.com/jetli/yew-hooks

use std::ops::Deref;
use std::{future::Future, rc::Rc};

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use yew_hooks::{use_mount, use_mut_latest};

/// Options for [`use_async_with_options`].
#[derive(Default)]
pub struct UseAsyncOptions {
    pub auto: bool,
}

impl UseAsyncOptions {
    /// Automatically run when mount
    pub const fn enable_auto() -> Self {
        Self { auto: true }
    }
}

/// State for an async future.
#[derive(PartialEq, Eq)]
pub enum UseAsyncState<T, E> {
    Pending,
    Processing,
    Ready(Result<T, E>),
}

impl<T, E> Default for UseAsyncState<T, E> {
    fn default() -> Self {
        Self::Pending
    }
}

impl<T, E> UseAsyncState<T, E> {
    #[inline]
    pub fn is_processing(&self) -> bool {
        matches!(self, Self::Processing)
    }
}

/// State handle for the [`use_async`] hook.
pub struct UseAsyncHandle<T, E> {
    inner: UseStateHandle<UseAsyncState<T, E>>,
    run: Rc<dyn Fn()>,
}

impl<T, E> UseAsyncHandle<T, E> {
    /// Start to resolve the async future to a final value.
    pub fn run(&self) {
        (self.run)();
    }

    /// Update `data` directly.
    pub fn update(&self, data: T) {
        self.inner.set(UseAsyncState::Ready(Ok(data)));
    }
}

impl<T, E> Deref for UseAsyncHandle<T, E> {
    type Target = UseAsyncState<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, E> Clone for UseAsyncHandle<T, E>
where
    T: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            run: self.run.clone(),
        }
    }
}

impl<T, E> PartialEq for UseAsyncHandle<T, E>
where
    T: PartialEq,
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook returns state and a `run` callback for an async future.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Async)]
/// fn async_test() -> Html {
///     let state = use_async(async move {
///         fetch("/api/user/123".to_string()).await
///     });
///
///     let onclick = {
///         let state = state.clone();
///         Callback::from(move |_| {
///             state.run();
///         })
///     };
///     
///     html! {
///         <div>
///             <button {onclick} disabled={state.is_processing()}>{ "Start loading" }</button>
///             {
///                 match *state {
///                     UseAsyncState::Pending => html! {},
///                     UseAsyncState::Loading => html! { "Loading" },
///                     UseAsyncState::Ready(Ok(data)) => html! { data },
///                     UseAsyncState::Ready(Err(error)) => html! { error },
///                 }
///             }
///         </div>
///     }
/// }
///
/// async fn fetch(url: String) -> Result<String, String> {
///     // You can use reqwest to fetch your http api
///     Ok(String::from("Jet Li"))
/// }
/// ```
#[hook]
pub fn use_async<F, T, E>(future: F) -> UseAsyncHandle<T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: 'static,
    E: 'static,
{
    use_async_with_options(future, UseAsyncOptions::default())
}

/// This hook returns state and a `run` callback for an async future with options.
/// See [`use_async`] too.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Async)]
/// fn async_test() -> Html {
///     let state = use_async_with_options(async move {
///         fetch("/api/user/123".to_string()).await
///     }, UseAsyncOptions::enable_auto());
///     
///     html! {
///         <div>
///             {
///                 match *state {
///                     UseAsyncState::Pending => html! {},
///                     UseAsyncState::Loading => html! { "Loading" },
///                     UseAsyncState::Ready(Ok(data)) => html! { data },
///                     UseAsyncState::Ready(Err(error)) => html! { error },
///                 }
///             }
///         </div>
///     }
/// }
///
/// async fn fetch(url: String) -> Result<String, String> {
///     // You can use reqwest to fetch your http api
///     Ok(String::from("Jet Li"))
/// }
/// ```
#[hook]
pub fn use_async_with_options<F, T, E>(future: F, options: UseAsyncOptions) -> UseAsyncHandle<T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: 'static,
    E: 'static,
{
    let inner = use_state(UseAsyncState::default);
    let future_ref = use_mut_latest(Some(future));

    let run = {
        let inner = inner.clone();
        Rc::new(move || {
            let inner = inner.clone();
            let future_ref = future_ref.clone();
            spawn_local(async move {
                let future_ref = future_ref.current();
                let future = (*future_ref.borrow_mut()).take();

                if let Some(future) = future {
                    // Set state to processing
                    inner.set(UseAsyncState::Processing);
                    inner.set(UseAsyncState::Ready(future.await));
                }
            });
        })
    };

    {
        let run = run.clone();
        use_mount(move || {
            if options.auto {
                run();
            }
        });
    }

    UseAsyncHandle { inner, run }
}

/// State handle for the [`use_async`] hook.
pub struct UseAsyncHandleDeps<T, E> {
    inner: UseStateHandle<UseAsyncState<T, E>>,
}

impl<T, E> Deref for UseAsyncHandleDeps<T, E> {
    type Target = UseAsyncState<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[hook]
pub fn use_async_with_deps<F, T, E, D, Fut>(f: F, deps: D) -> UseAsyncHandleDeps<T, E>
where
    F: FnOnce(&D) -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
    T: 'static,
    E: 'static,
    D: PartialEq + 'static,
{
    let inner = use_state(UseAsyncState::default);

    let run = {
        let inner = inner.clone();
        Rc::new(move |future| {
            let inner = inner.clone();
            spawn_local(async move {
                // Set state to processing
                inner.set(UseAsyncState::Processing);
                // Process and update
                inner.set(UseAsyncState::Ready(future.await));
            });
        })
    };

    let future_ref = use_mut_latest(Some(f));

    {
        let run = run.clone();
        use_effect_with_deps(
            move |deps| {
                let future_ref = future_ref.current();
                let future = (*future_ref.borrow_mut()).take();

                if let Some(future) = future {
                    run(future(&deps));
                }
            },
            deps,
        )
    };

    UseAsyncHandleDeps { inner }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        async fn fetch(value: &str) -> Result<String, ()> {
            Ok(format!("foo/{value}"))
        }

        #[function_component(Test)]
        fn test() -> Html {
            let props = String::new();

            let fetch = use_async_with_deps(
                |props| {
                    let props = props.clone();
                    async move { fetch(&props).await }
                },
                props.clone(),
            );
            match &*fetch {
                UseAsyncState::Pending | UseAsyncState::Processing => html!(),
                UseAsyncState::Ready(_) => html!(),
            }
        }

        let _html = html!(<Test/>);
    }
}
