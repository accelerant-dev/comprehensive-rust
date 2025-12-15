# PhantomPinned

The idiomatic way to opt-out of Rust's aliasing

Usage

```rust,editable
pub struct DynamicBuffer {
    data: Vec<u8>,
    cursor: NonNull<u8>,
    _pin: std::marker::PhantomPinned,
}

impl DynamicBuffer {
    pub fn push(&mut self, byte: u8) {
        self.data.push(byte);

        // TODO: self.data may have reallocated; ensure that th cursorpoints the the correct place
    }
}
```

<details>

If a type contains a `PhantomPinned`, it will not implement `Unpin` by default.

<!-- TODO: Monitor issue https://github.com/rust-lang/rust/issues/125735 as this guidance will change at some point and future code will move to UnsafePinned -->

</details>
