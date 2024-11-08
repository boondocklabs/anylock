//! Implementation for [`tokio::sync::Mutex`] and [`tokio::sync::RwLock`]

use crate::AnyLock;
use std::future::Future;

/// Tokio Mutex
pub struct TokioMutex<T>(tokio::sync::Mutex<T>);

impl<T> AnyLock<T> for TokioMutex<T> {
    type ReadGuard<'a> = tokio::sync::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = tokio::sync::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.blocking_lock()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.blocking_lock()
    }

    fn async_read<'a>(&'a self) -> Box<dyn Future<Output = Self::ReadGuard<'a>> + 'a> {
        Box::new(async move { self.0.lock().await })
    }

    fn async_write<'a>(&'a self) -> Box<dyn Future<Output = Self::WriteGuard<'a>> + 'a> {
        Box::new(async move { self.0.lock().await })
    }

    /// Create a new lock [`tokio::sync::Mutex`]
    fn new(inner: T) -> Self {
        TokioMutex(tokio::sync::Mutex::new(inner))
    }
}

impl<T> std::fmt::Debug for TokioMutex<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tokio RwLock
pub struct TokioRwLock<T>(tokio::sync::RwLock<T>);

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

impl<T> std::fmt::Debug for TokioRwLock<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
