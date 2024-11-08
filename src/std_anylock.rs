//! Implementation for [`std::sync::Mutex`] and [`std::sync::RwLock`]

use crate::AnyLock;

pub struct StdMutex<T>(std::sync::Mutex<T>);

impl<T> AnyLock<T> for StdMutex<T> {
    type ReadGuard<'a> = std::sync::MutexGuard<'a, T>
    where T: 'a, Self: 'a;
    type WriteGuard<'a> = std::sync::MutexGuard<'a, T>
    where T: 'a, Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.lock().expect("Failed to acquire Mutex")
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.lock().expect("Failed to acquire Mutex")
    }

    /// Create a new lock [`std::sync::Mutex`]
    fn new(inner: T) -> Self {
        StdMutex(std::sync::Mutex::new(inner))
    }
}

impl<T> std::fmt::Debug for StdMutex<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Wrapped [`std::sync::RwLock`]
pub struct StdRwLock<T>(std::sync::RwLock<T>);

impl<T> AnyLock<T> for StdRwLock<T> {
    type ReadGuard<'a> = std::sync::RwLockReadGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = std::sync::RwLockWriteGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.read().expect("Failed to acquire read guard")
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.write().expect("Failed to acquire write guard")
    }

    /// Create a new lock [`std::sync::RwLock`]
    fn new(inner: T) -> Self {
        StdRwLock(std::sync::RwLock::new(inner))
    }
}

impl<T> std::fmt::Debug for StdRwLock<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
