//! Implementation for [`parking_lot::Mutex`] and [`parking_lot::RwLock`]

use crate::AnyLock;

impl<T> AnyLock<T> for parking_lot::Mutex<T> {
    type ReadGuard<'a> = parking_lot::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    type WriteGuard<'a> = parking_lot::MutexGuard<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.lock()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.lock()
    }

    /// Create a new lock [`parking_lot::Mutex`]
    fn new(inner: T) -> Self {
        parking_lot::Mutex::new(inner)
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
