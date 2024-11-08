//! Implementation for [`parking_lot::Mutex`] and [`parking_lot::RwLock`]

use crate::AnyLock;

pub struct ParkingLotMutex<T>(parking_lot::Mutex<T>);

impl<T> AnyLock<T> for ParkingLotMutex<T> {
    type ReadGuard<'a> = parking_lot::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = parking_lot::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.lock()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.lock()
    }

    /// Create a new lock [`parking_lot::Mutex`]
    fn new(inner: T) -> Self {
        ParkingLotMutex(parking_lot::Mutex::new(inner))
    }
}

impl<T> std::fmt::Debug for ParkingLotMutex<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct ParkingLotRwLock<T>(parking_lot::RwLock<T>);

impl<T> AnyLock<T> for ParkingLotRwLock<T> {
    type ReadGuard<'a> = parking_lot::RwLockReadGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = parking_lot::RwLockWriteGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.read()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.write()
    }

    /// Create a new lock [`parking_lot::RwLock`]
    fn new(inner: T) -> Self {
        ParkingLotRwLock(parking_lot::RwLock::new(inner))
    }
}

impl<T> std::fmt::Debug for ParkingLotRwLock<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
