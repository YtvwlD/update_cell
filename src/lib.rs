//! A `Cell<Option<T>>` that you can update
//!
//! # Why would I need this?
//!
//! [`Cell::update`](https://doc.rust-lang.org/core/cell/struct.Cell.html#method.update)
//! only supports types that are `Copy`. So if you want to store and modify
//! something that is neither `Copy` nor has a `Default` (eg. a builder),
//! this crate might be useful.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! update_cell = "0.1"
//! ```
//!
//! And if you have a struct, you can put it inside and modify it:
//!
//! ```rust
//! use update_cell::UpdateCell;
//!
//! struct MySuperFancyStruct {
//!     inner: bool
//! }
//!
//! impl MySuperFancyStruct {
//!     fn new() -> Self {
//!         Self { inner: false }
//!     }
//!
//!     fn toggle(mut self) -> Self {
//!         self.inner = !self.inner;
//!         self
//!     }
//! }
//!
//! let mut cell = UpdateCell::new(MySuperFancyStruct::new());
//! cell.update(|s| s.toggle());
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/update_cell/0.1.0")]
use core::cell::Cell;

/// The main struct.
///
/// Put whatever you want in here.
pub struct UpdateCell<T> {
    value: Cell<Option<T>>,
}

impl<T> UpdateCell<T> {
    /// Create a new instance.
    pub fn new(val: T) -> Self {
        Self {
            value: Cell::new(Some(val)),
        }
    }

    /// Modify the contained value by passing a function.
    pub fn update<F: FnOnce(T) -> T>(&mut self, func: F) {
        let val = self.value.take().unwrap();
        self.value.set(Some(func(val)))
    }

    /// Resolve the cell into the inner value.
    pub fn into_inner(self) -> T {
        self.value.take().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Foo {
        inner: bool,
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
