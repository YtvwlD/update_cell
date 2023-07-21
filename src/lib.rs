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

#[cfg(test)]
mod tests {
    use super::*;
    struct Foo {
        inner: bool
    }

    impl Foo {
        fn new() -> Self {
            Self { inner: false }
        }

        fn turn_on(mut self) -> Self {
            self.inner = true;
            self
        }

        fn get(&self) -> bool {
            self.inner
        }
    }

    #[test]
    fn new() {
        let _cell = UpdateCell::new(Foo::new());
    }

    #[test]
    fn into() {
        let cell = UpdateCell::new(Foo::new());
        let foo: Foo = cell.into_inner();
        assert!(!foo.get());
    }

    #[test]
    fn end_to_end() {
        let mut cell = UpdateCell::new(Foo::new());
        cell.update(|f| f.turn_on());
        let foo: Foo = cell.into_inner();
        assert!(foo.get());
    }
}

