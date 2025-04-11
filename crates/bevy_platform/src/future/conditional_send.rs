pub use send_bounds::ConditionalSend;

#[cfg(target_arch = "wasm32")]
mod send_bounds {
    /// This will require [`Send`] on platforms where [`Future`] is [`Send`].
    /// On wasm32, this will not be [`Send`].
    pub trait ConditionalSend {}
    impl<T> ConditionalSend for T {}
}

#[cfg(not(target_arch = "wasm32"))]
mod send_bounds {
    /// This will require [`Send`] on platforms where [`Future`] is [`Send`].
    /// On wasm32, this will not be [`Send`].
    pub trait ConditionalSend: Send {}
    impl<T: Send> ConditionalSend for T {}
}
