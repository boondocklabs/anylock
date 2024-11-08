use std::{marker::PhantomData, sync::Arc};

use crate::AnyLock;

#[derive(Default, Debug)]
struct MyWrapper<T, Inner>
where
    T: std::fmt::Debug,
    Inner: AnyLock<T>,
{
    inner: Arc<Inner>,
    _phantom: PhantomData<T>,
}

impl<T, Lock> MyWrapper<T, Lock>
where
    T: std::fmt::Debug,
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
    let x = MyWrapper::<String, crate::StdMutex<String>>::new("Hello".into());
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn core_refcell() {
    let x = MyWrapper::<String, crate::CoreRefCell<String>>::new("Hello".into());
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn parking_lot_mutex() {
    let x = MyWrapper::<String, crate::ParkingLotMutex<String>>::new("Hello".into());
    println!("{:?}", x.inner.read());
}

#[test]
pub fn tokio_sync_mutex() {
    let x = MyWrapper::<String, crate::TokioMutex<String>>::new("Hello".into());
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn std_rwlock() {
    let x = MyWrapper::<String, crate::StdRwLock<String>>::new("Hello".into());
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn parking_lot_rwlock_read() {
    let x = MyWrapper::<String, crate::ParkingLotRwLock<String>>::new("Hello".into());
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn parking_lot_rwlock_write() {
    let x = MyWrapper::<String, crate::ParkingLotRwLock<String>>::new("Hello".into());
    *x.inner.write() = "World".into();
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn tokio_sync_rwlock_read() {
    let x = MyWrapper::<String, crate::TokioRwLock<String>>::new("Hello".into());
    println!("{:#?}", x);
    println!("{:?}", x.inner.read());
}

#[test]
pub fn tokio_sync_rwlock_write() {
    let x = MyWrapper::<String, crate::TokioRwLock<String>>::new("Hello".into());
    *x.inner.write() = "World".into();
    println!("{:#?}", x);
    println!("{:?}", x.inner.write());
}
