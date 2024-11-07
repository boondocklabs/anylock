//! Implementation for [`tokio::sync::Mutex`] and [`tokio::sync::RwLock`]

use crate::AnyLock;
use std::future::Future;

/// Tokio Mutex
impl<T> AnyLock<T> for tokio::sync::Mutex<T> {
    type ReadGuard<'a> = tokio::sync::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = tokio::sync::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.blocking_lock()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.blocking_lock()
    }

    fn async_read<'a>(&'a self) -> Box<dyn Future<Output = Self::ReadGuard<'a>> + 'a> {
        Box::new(async move { self.lock().await })
    }

    fn async_write<'a>(&'a self) -> Box<dyn Future<Output = Self::WriteGuard<'a>> + 'a> {
        Box::new(async move { self.lock().await })
    }

    /// Create a new lock [`tokio::sync::Mutex`]
    fn new(inner: T) -> Self {
        tokio::sync::Mutex::new(inner)
    }
}

/// Tokio RwLock

pub struct TokioRwLock<T>(tokio::sync::RwLock<T>);

/*
impl AnyLockInner for TokioRwLock {
    type Inner = Box<dyn std::any::Any + Sync>;
}
*/

impl<T> AnyLock<T> for TokioRwLock<T> {
    type ReadGuard<'a> = tokio::sync::RwLockReadGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = tokio::sync::RwLockWriteGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.blocking_read()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.blocking_write()
    }

    fn async_read<'a>(&'a self) -> Box<dyn Future<Output = Self::ReadGuard<'a>> + 'a> {
        Box::new(async move { self.0.read().await })
    }

    fn async_write<'a>(&'a self) -> Box<dyn Future<Output = Self::WriteGuard<'a>> + 'a> {
        Box::new(async move { self.0.write().await })
    }

    /// Create a new lock [`tokio::sync::RwLock`]
    fn new(inner: T) -> Self {
        TokioRwLock(tokio::sync::RwLock::new(inner))
    }
}
