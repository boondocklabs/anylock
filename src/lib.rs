//! # AnyLock Polymorphic Lock Traits
//!
//! Provides a unified interface to different underlying lock implementations.
//!
//! Works as either `RwLock` or `Mutex`. For `Mutex` types, [`AnyLock::read()`] and [`AnyLock::write()`] are aliased.
//!
//! [`AnyLock`] provides newtype wrappers around supported locks:
//!
//! | Provider       | Mutex                           | RwLock                       |
//! |----------------|---------------------------------|------------------------------|
//! | parking_lot    | [`crate::ParkingLotMutex`]      | [`crate::ParkingLotRwLock`]  |
//! | std::sync      | [`crate::StdMutex`]             | [`crate::StdRwLock`]         |
//! | tokio          | [`crate::TokioMutex`]           | [`crate::TokioRwLock`]       |
//! | core::sync     | [`crate::CoreRefCell`]          |                              |
//!
//! ## Async
//! Async locks using [`tokio`] are supported using [`AnyLock::async_read()`] and [`AnyLock::async_write()`].
//!
//! ## Example
//!
//! ```
//! use anylock::AnyLock;
//! use std::{marker::PhantomData, sync::Arc};
//!
//! // Using type annotations, you can use AnyLock::new()
//! let arc_lock_string: Arc<anylock::StdMutex<String>> =
//!     Arc::new(AnyLock::new("Hello AnyLock".into()));
//!
//! // Although this is a Mutex, we access it using read() and write()
//! // to allow AnyLock to be compatible with RwLock APIs.
//! println!("{}", arc_lock_string.read());
//!
//! /// Example struct wrapping some inner type T with any kind of lock
//! #[derive(Default)]
//! struct MyWrapper<T, Lock>
//! where
//!     // Use a trait bound to accept any kind of lock
//!     Lock: AnyLock<T>,
//! {
//!     inner: Arc<Lock>,
//!     _phantom: PhantomData<T>,
//! }
//!
//! impl<T, Lock> MyWrapper<T, Lock>
//! where
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
//! let x = MyWrapper::<String, anylock::StdMutex<_>>::new("Hello".into());
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

pub mod core_cell;
pub mod parking_lot_anylock;
pub mod std_anylock;
pub mod tokio_anylock;

pub use core_cell::CoreRefCell;
pub use parking_lot_anylock::{ParkingLotMutex, ParkingLotRwLock};
pub use std_anylock::{StdMutex, StdRwLock};
pub use tokio_anylock::{TokioMutex, TokioRwLock};

#[cfg(test)]
mod tests;

use std::{
    future::Future,
    ops::{Deref, DerefMut},
};

/// AnyLock trait allows different underlying lock implementations.
///
///
pub trait AnyLock<T> {
    type ReadGuard<'a>: Deref<Target = T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a>: DerefMut<Target = T>
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

    fn async_read<'a>(&'a self) -> Box<dyn Future<Output = Self::ReadGuard<'a>> + 'a> {
        Box::new(async { panic!("Async read not supported") })
    }

    fn async_write<'a>(&'a self) -> Box<dyn Future<Output = Self::WriteGuard<'a>> + 'a> {
        Box::new(async { panic!("Async write not supported") })
    }
}
