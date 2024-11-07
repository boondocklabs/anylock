use std::{marker::PhantomData, sync::Arc};

use crate::AnyLock;

#[derive(Default)]
struct MyWrapper<T, Inner>
where
    T: Send + Sync,
    Inner: AnyLock<T>,
{
    inner: Arc<Inner>,
    _phantom: PhantomData<T>,
}

impl<T, Lock> MyWrapper<T, Lock>
where
    T: Send + Sync,
    Lock: AnyLock<T>,
{
    fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(Lock::new(inner)),
            _phantom: PhantomData,
        }
    }
}

#[test]
pub fn std_mutex() {
    let x = MyWrapper::<String, std::sync::Mutex<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn parking_lot_mutex() {
    let x = MyWrapper::<String, parking_lot::Mutex<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn tokio_sync_mutex() {
    let x = MyWrapper::<String, tokio::sync::Mutex<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn std_rwlock() {
    let x = MyWrapper::<String, crate::StdRwLock<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn parking_lot_rwlock_read() {
    let x = MyWrapper::<String, crate::ParkingLotRwLock<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn parking_lot_rwlock_write() {
    let x = MyWrapper::<String, crate::ParkingLotRwLock<String>>::new("Hello".into());
    *x.inner.write() = "World".into();
    println!("{:?}", x.inner.read());
}

#[test]
pub fn tokio_sync_rwlock_read() {
    let x = MyWrapper::<String, crate::TokioRwLock<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn tokio_sync_rwlock_write() {
    let x = MyWrapper::<String, crate::TokioRwLock<String>>::new("Hello".into());
    *x.inner.write() = "World".into();
    println!("{:?}", x.inner.write());
}
