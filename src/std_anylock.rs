//! Implementation for [`std::sync::Mutex`] and [`std::sync::RwLock`]

use crate::AnyLock;

impl<T> AnyLock<T> for std::sync::Mutex<T> {
    type ReadGuard<'a> = std::sync::MutexGuard<'a, T>
    where T: 'a, Self: 'a;
    type WriteGuard<'a> = std::sync::MutexGuard<'a, T>
    where T: 'a, Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.lock().expect("Failed to acquire Mutex")
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.lock().expect("Failed to acquire Mutex")
    }

    /// Create a new lock [`std::sync::Mutex`]
    fn new(inner: T) -> Self {
        std::sync::Mutex::new(inner)
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
