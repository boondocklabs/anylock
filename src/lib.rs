//! # AnyLock Polymorphic Lock Traits
//!
//! Provides a unified interface to different underlying lock implementations.
//!
//! Works as either `RwLock` or `Mutex`. For `Mutex` types, [`AnyLock::read()`] and [`AnyLock::write()`] are aliased.
//!
//! <div class="warning">
//! Note that when using `RwLock` types, the newtype exported from AnyLock (StdRwLock, ParkingLotRwLock, TokioRwLock) must be used.
//! The impl of the AnyLock trait is only on the newtype wrappers to avoid collision of the `read()` and `write()` methods with the underlying RwLock implementations.
//! </div>
//!
//! ## Async
//! Async locks using [`tokio`] are supported using [`AnyLock::async_read()`] and [`AnyLock::async_write()`].
//!
//! ## Supported Lock Implementations
//!
//! * std::sync
//! * parking_lot
//! * tokio
//!
//! ## Example
//!
//! ```
//! use anylock::AnyLock;
//! use std::{marker::PhantomData, sync::Arc};
//!
//! /// Example struct wrapping some inner type T
//! #[derive(Default)]
//! struct MyWrapper<T, Lock>
//! where
//!     T: Send + Sync,
//!     Lock: AnyLock<T>,
//! {
//!     inner: Arc<Lock>,
//!     _phantom: PhantomData<T>,
//! }
//!
//! impl<T, Lock> MyWrapper<T, Lock>
//! where
//!     T: Send + Sync,
//!     Lock: AnyLock<T>,
//! {
//!     fn new(inner: T) -> Self {
//!         Self {
//!             inner: Arc::new(Lock::new(inner)),
//!             _phantom: PhantomData,
//!         }
//!     }
//! }
//!
//! // Now we can create MyWrapper with different locks without modifying
//! // the implementation of MyWrapper itself.
//!
//! // std::Mutex
//! let x = MyWrapper::<String, std::sync::Mutex<String>>::new("Hello".into());
//! println!("{}", x.inner.read());
//!
//! // parking_lot::RwLock
//! let x = MyWrapper::<String, anylock::ParkingLotRwLock<String>>::new("Hello".into());
//!
//! // Acquire write lock and write
//! *x.inner.write() = "World".into();
//!
//! // Acquire read lock and read
//! println!("{:?}", x.inner.read());
//!
//!
//! ```

pub mod parking_lot_anylock;
pub mod std_anylock;
pub mod tokio_anylock;

pub use parking_lot_anylock::ParkingLotRwLock;
pub use std_anylock::StdRwLock;
pub use tokio_anylock::TokioRwLock;

#[cfg(test)]
mod tests;

use std::{
    future::Future,
    ops::{Deref, DerefMut},
};

pub trait AnyLock<T>: Send + Sync
where
    T: Send + Sync,
{
    type ReadGuard<'a>: Deref<Target = T> + Sync
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a>: DerefMut<Target = T> + Sync
    where
        T: 'a,
        Self: 'a;

    fn new(inner: T) -> Self;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        panic!("Synchronous read not supported")
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        panic!("Synchronous read not supported")
    }

    fn async_read<'a>(&'a self) -> Box<dyn Future<Output = Self::ReadGuard<'a>> + Send + 'a> {
        Box::new(async { panic!("Async read not supported") })
    }

    fn async_write<'a>(&'a self) -> Box<dyn Future<Output = Self::WriteGuard<'a>> + Send + 'a> {
        Box::new(async { panic!("Async write not supported") })
    }
}
