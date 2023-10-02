//! Open URLs

use yew::prelude::*;

/// Open a URL in the provided target.
///
/// This provides a callback, which opens the `url` in the provided `target`, using
/// [`web_sys::Window::open_with_url_and_target`].
#[hook]
pub fn use_open<IN>(url: impl Into<String>, target: impl Into<String>) -> Callback<IN, ()>
where
    IN: 'static,
{
    use_callback((url.into(), target.into()), |_, (url, target)| {
        let _ = gloo_utils::window().open_with_url_and_target(url, target);
    })
}
