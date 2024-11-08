//! Implementation for [`core::cell::RefCell`]

use crate::AnyLock;

pub struct CoreRefCell<T>(core::cell::RefCell<T>);

impl<T> AnyLock<T> for CoreRefCell<T> {
    type ReadGuard<'a> = core::cell::Ref<'a, T>
    where T: 'a, Self: 'a;
    type WriteGuard<'a> = core::cell::RefMut<'a, T>
    where T: 'a, Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.0.borrow()
    }

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.0.borrow_mut()
    }

    /// Create a new lock [`std::sync::Mutex`]
    fn new(inner: T) -> Self {
        CoreRefCell(core::cell::RefCell::new(inner))
    }
}

impl<T> std::fmt::Debug for CoreRefCell<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
