# update_cell

A `Cell<Option<T>>` that you can update

## Why would I need this?

[`Cell::update`] is currently experimental. And it only supports types
that are `Copy`. So if you want to store and modify something that is
neither `Copy` nor has a `Default` (eg. a builder), this crate might be
useful.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
update_cell = "0.1"
```

And if you have a struct, you can put it inside and modify it:

```rust
use update_cell::UpdateCell;

struct MySuperFancyStruct {
    inner: bool
}

impl MySuperFancyStruct {
    fn new() -> Self {
        Self { inner: false }
    }

    fn toggle(mut self) -> Self {
        self.inner = !self.inner;
        self
    }
}

let mut cell = UpdateCell::new(MySuperFancyStruct::new());
cell.update(|s| s.toggle());
```

License: MPL-2.0
