use yew::prelude::*;
use yew_hooks::use_event_with_window;

/// A trait characterizing breakpoints for viewport sizes.
///
/// ## Example
///
/// ```
/// use yew_more_hooks::prelude::Breakpoint;
///
/// #[derive(PartialEq, Clone, Copy)]
/// enum AppBreakpoint {
///     Small,
///     Large,
/// }
///
/// impl Breakpoint for AppBreakpoint {
///     fn from_screen_width(pixels: usize) -> Self {
///         if pixels < Self::Large.as_pixels() {
///             Self::Small
///         } else {
///             Self::Large
///         }
///     }
///
///     fn as_pixels(&self) -> usize {
///         match self {
///             Self::Small => 0,
///             Self::Large => 1280,
///         }
///     }
/// }
/// ```
pub trait Breakpoint: PartialEq + Sized {
    fn as_pixels(&self) -> usize;

    fn from_screen_width(pixels: usize) -> Self;

    fn current() -> Self {
        let width = web_sys::window()
            .expect("Couldn't get window")
            .inner_width()
            .expect("Couldn't retrieve width of window")
            .as_f64()
            .expect("Couldn't convert window size to number")
            .round();
        Self::from_screen_width(width as usize)
    }
}

/// Causes a rerender of a component when the viewport width is changed beyond a breakpoint.
///
/// ## Example
/// ```
/// # use yew_more_hooks::prelude::Breakpoint;
/// use yew_more_hooks::prelude::use_breakpoint;
/// use yew::prelude::*;
/// #
/// # #[derive(PartialEq, Clone, Copy)]
/// # enum AppBreakpoint {
/// #    Small,
/// #    Large,
/// # }
/// #
/// # impl Breakpoint for AppBreakpoint {
/// #    fn from_screen_width(pixels: usize) -> Self {
/// #        if pixels < Self::Large.as_pixels() {
/// #            Self::Small
/// #        } else {
/// #            Self::Large
/// #        }
/// #    }
/// #
/// #    fn as_pixels(&self) -> usize {
/// #        match self {
/// #            Self::Small => 0,
/// #            Self::Large => 1280,
/// #        }
/// #    }
/// # }
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     let breakpoint = use_breakpoint::<AppBreakpoint>();
///     let label = use_memo(breakpoint.clone(), |breakpoint| match **breakpoint {
///         AppBreakpoint::Small => html!("Label"),
///         AppBreakpoint::Large => html!("A very long descriptive label"),
///     });
///     (*label).clone()
/// }
/// ```
#[hook]
pub fn use_breakpoint<T: Breakpoint + 'static>() -> UseStateHandle<T> {
    let state: UseStateHandle<T> = use_state_eq(Breakpoint::current);
    {
        let state = state.clone();
        use_event_with_window("resize", move |_: Event| {
            state.set(Breakpoint::current());
        });
    }
    state.clone()
}
