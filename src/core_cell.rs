//! Implementation for [`core::cell::RefCell`]

use crate::AnyLock;

impl<T> AnyLock<T> for core::cell::RefCell<T>
where
    T: std::fmt::Debug,
{
    type ReadGuard<'a> = core::cell::Ref<'a, T>
    where T: 'a, Self: 'a;
    type WriteGuard<'a> = core::cell::RefMut<'a, T>
    where T: 'a, Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.borrow()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.borrow_mut()
    }

    /// Create a new lock [`std::sync::Mutex`]
    fn new(inner: T) -> Self {
        core::cell::RefCell::new(inner)
    }
}
