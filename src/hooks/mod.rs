//! Additional hooks for the Yew hook system

#[cfg(feature = "yew-hooks")]
pub mod r#async;
pub mod open;

pub use open::*;
#[cfg(feature = "yew-hooks")]
pub use r#async::*;
