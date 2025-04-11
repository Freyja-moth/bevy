#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://bevyengine.org/assets/icon.png",
    html_favicon_url = "https://bevyengine.org/assets/icon.png"
)]
#![no_std]

//! Platform compatibility support for first-party [Bevy] engine crates.
//!
//! [Bevy]: https://bevyengine.org/

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod future;
pub mod hash;
pub mod sync;
pub mod thread;
pub mod time;
if_web!(
    pub mod web;
);

#[cfg(feature = "alloc")]
pub mod collections;

pub use uuid;
/// Frequently used items which would typically be included in most contexts.
///
/// When adding `no_std` support to a crate for the first time, often there's a substantial refactor
/// required due to the change in implicit prelude from `std::prelude` to `core::prelude`.
/// This unfortunately leaves out many items from `alloc`, even if the crate unconditionally
/// includes that crate.
///
/// This prelude aims to ease the transition by re-exporting items from `alloc` which would
/// otherwise be included in the `std` implicit prelude.
pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use alloc::{
        borrow::ToOwned, boxed::Box, format, string::String, string::ToString, vec, vec::Vec,
    };

    // Items from `std::prelude` that are missing in this module:
    // * dbg
    // * eprint
    // * eprintln
    // * is_x86_feature_detected
    // * print
    // * println
    // * thread_local
}

/// This is true when the "web" cargo feature is enabled
pub const IS_WEB: bool = cfg!(feature = "web");

/// This is true when the "std" cargo feature is enabled
pub const IS_STD: bool = cfg!(feature = "std");

/// compiles the given code if the "web" feature is enabled
#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[macro_export]
macro_rules! if_web {
    ($($tt:tt)*) => {$($tt)*};
}

/// compiles the given code if the "web" feature is enabled
#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
#[macro_export]
macro_rules! if_web {
    ($($tt:tt)*) => {};
}

/// compiles the given code if the "web" feature is disabled
#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
#[macro_export]
macro_rules! if_not_web {
    ($($tt:tt)*) => {$($tt)*};
}

/// compiles the given code if the "web" feature is disabled
#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[macro_export]
macro_rules! if_not_web {
    ($($tt:tt)*) => {};
}

/// compiles the given code if the "web" feature is disabled
#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[macro_export]
macro_rules! if_web_else {
    ({$($tt_if:tt)*}, {$($tt_else:tt)*}) => {{$($tt_if)*}};
}

/// compiles the given code if the "web" feature is disabled
#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
#[macro_export]
macro_rules! if_web_else {
    ({$($tt_if:tt)*}, {$($tt_else:tt)*}) => {{$($tt_else)*}};
}

/// compiles the given code if the "std" feature is enabled
#[cfg(feature = "std")]
#[macro_export]
macro_rules! if_std {
    ($($tt:tt)*) => {$($tt)*};
}

/// compiles the given code if the "std" feature is enabled
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! if_std {
    ($($tt:tt)*) => {};
}
