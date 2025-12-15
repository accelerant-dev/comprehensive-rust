# Unpin trait

- `T: Unpin` implies that `T` is not pinned
- Automatically implemented by the compiler for nearly every type
- To opt out of this for your type, add a [`PhantomPinned`] field to your type
  (required for FFI)

<details>

Most types implement `Unpin` automatically `Unpin` types can be moved even when
pinned

`!Unpin` types cannot be moved once pinned

Unpin is a promise: "moving me is always safe"

</details>

[`Pantom`]: https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html
