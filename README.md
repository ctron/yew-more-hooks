# More Yew hooks

[![crates.io](https://img.shields.io/crates/v/yew-more-hooks.svg)](https://crates.io/crates/yew-more-hooks)
[![docs.rs](https://docs.rs/yew-more-hooks/badge.svg)](https://docs.rs/yew-more-hooks)

More hooks for [Yew](https://yew.rs/):

> Yew is a framework for creating reliable and efficient web applications.

Also see [jetli/yew-hooks](https://github.com/jetli/yew-hooks) for many more Yew Hooks.

## Hooks

* `use_async` – Perform an async operation. This is a variation of [jetli/yew-hooks](https://github.com/jetli/yew-hooks)' `use_async`, supporting dependencies and a more ergonomic enum to track the outcome.
* `use_open` – Open a link
* `use_page_state` – Work with the state of a page
* `use_reform` - Reform a callback, keeping a stable instance
* `use_map` - A combination of `Option::map` and `use_memo`