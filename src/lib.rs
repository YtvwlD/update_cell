#![no_std]
use core::cell::Cell;

pub struct UpdateCell<T> {
    value: Cell<Option<T>>
}

impl<T> UpdateCell<T> {
    pub fn new(val: T) -> Self {
        Self { value: Cell::new(Some(val)) }
    }
    
    pub fn update<F: FnOnce(T) -> T>(&mut self, func: F) {
        let val = self.value.take().unwrap();
        self.value.set(Some(func(val)))
    }

    pub fn into_inner(self) -> T {
        self.value.take().unwrap()
    }
}
