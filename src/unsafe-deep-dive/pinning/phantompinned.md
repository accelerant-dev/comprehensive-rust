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
        // Calculate the cursor offset before the push (which may reallocate)
        let offset = unsafe {
            self.cursor.as_ptr().offset_from(self.data.as_ptr())
        };
        
        self.data.push(byte);

        // Update cursor to point to the same offset in the (potentially new) buffer
        self.cursor = unsafe {
            NonNull::new_unchecked(self.data.as_mut_ptr().offset(offset))
        };
    }
}
```

<details>

If a type contains a `PhantomPinned`, it will not implement `Unpin` by default.

<!-- TODO: Monitor issue https://github.com/rust-lang/rust/issues/125735 as this guidance will change at some point and future code will move to UnsafePinned -->

</details>
