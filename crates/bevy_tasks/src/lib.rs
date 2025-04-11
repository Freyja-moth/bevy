#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://bevyengine.org/assets/icon.png",
    html_favicon_url = "https://bevyengine.org/assets/icon.png"
)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub mod futures;
/// The tasks prelude.
///
/// This includes the most common types in this crate, re-exported for your convenience.
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        iter::ParallelIterator,
        slice::{ParallelSlice, ParallelSliceMut},
        usages::{AsyncComputeTaskPool, ComputeTaskPool, IoTaskPool},
    };

    #[cfg(feature = "std")]
    #[doc(hidden)]
    pub use crate::block_on;
}

#[cfg(not(feature = "async_executor"))]
mod edge_executor;
mod executor;
mod iter;
mod slice;
mod usages;
if_web!(
    mod wasm_task;
    pub use wasm_task::Task;
);
if_not_web!(
    mod task;
    pub use task::Task;
);

pub use futures_lite;
pub use futures_lite::future::poll_once;
pub use iter::ParallelIterator;
pub use slice::*;
pub use usages::*;

use bevy_platform::{if_not_web, if_web};

cfg_if::cfg_if! {
    if #[cfg(all(not(target_arch = "wasm32"), feature = "multi_threaded"))] {
        mod task_pool;
        mod thread_executor;

        pub use task_pool::{Scope, TaskPool, TaskPoolBuilder};
        pub use thread_executor::{ThreadExecutor, ThreadExecutorTicker};
    } else if #[cfg(any(target_arch = "wasm32", not(feature = "multi_threaded")))] {
        mod single_threaded_task_pool;

        pub use single_threaded_task_pool::{Scope, TaskPool, TaskPoolBuilder, ThreadExecutor};
    }
}

#[cfg(feature = "std")]
cfg_if::cfg_if! {
    if #[cfg(feature = "async-io")] {
        pub use async_io::block_on;
    } else {
        pub use futures_lite::future::block_on;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use core::num::NonZero;

        /// Gets the logical CPU core count available to the current process.
        ///
        /// This is identical to [`std::thread::available_parallelism`], except
        /// it will return a default value of 1 if it internally errors out.
        ///
        /// This will always return at least 1.
        pub fn available_parallelism() -> usize {
            std::thread::available_parallelism()
                .map(NonZero::<usize>::get)
                .unwrap_or(1)
        }
    } else {
        /// Gets the logical CPU core count available to the current process.
        ///
        /// This will always return at least 1.
        pub fn available_parallelism() -> usize {
            // Without access to std, assume a single thread is available
            1
        }
    }
}
