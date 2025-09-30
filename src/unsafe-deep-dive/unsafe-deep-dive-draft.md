---
marp: true
theme: default
title: Unsafe Deep Dive
---

# Whole course draft

> Note
>
> This file is an attempt to create a single document that contains the course
> content. It will then be refactored into multiple slides.
>
> Italics are used to indicate the first time a term is used.

# Whole unsafe deep dive course draft

## Day 1: Morning

---

# Aims

- increase knowledge of
- increase your ability to use unsafe
- self-review easy cases

---

# Your aims

What are your goals for this class?

<details>

_Script_

Is there anything that you would like to make sure that we've talked about
during this session?

It's likely that we'll cover your points through the course content. If not, we
should have some buffer time available to address anything else.

_Aims for slide_

- Address any latent concerns that the course won't be relevant to the learner
- Ensure that the content and delivery can be tweaked to meet the learners'
  needs

</details>

---

# Warm up

3 examples:

- using an unsafe block
- defining an unsafe function
- implementing an unsafe trait

<details>

_Script_

We'll start by going through a couple of examples that introduce the mechanics
of using unsafe.

If you have any questions, then please note them down. This will generate a
checklist that we'll make sure that we've covered off.

_Advice for the next few slides_

Show the mechanics such as which keywords go where and what constructs are
possible. Avoid deep explanations.

</details>

---

# Warm up: using an unsafe block

```rust
// TODO: fix the compiler errors

fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut _;
    let b: *mut i32 = std::ptr::null_mut();

    println!("{:?}", a.as_mut());
    println!("{:?}", b.as_mut());
}
```

<details>

_Instructions_

- Introduce `.as_mut()`; converts pointers to Options
- Wrap the calls in unsafe
- Add a safety comment above the two calls saying that we know that the memory
  is valid

</details>

---

# Warm up: defining an unsafe function

```rust
// TODO: mark ptr_to_option as unsafe and document any

/// Convert a pointer to an `Option<T>`
///
/// Returns `None` when `val` is null, otherwise wraps `val` in `Some`.
fn ptr_to_option<'a, T>(val: *mut T) -> Option<&'a mut T> {
    if val.is_null() { None } else { unsafe { Some(&mut *val) } }
}

fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut _;
    let b: *mut i32 = std::ptr::null_mut();

    println!("{:?}", ptr_to_option(a));
    println!("{:?}", ptr_to_option(a));
}
```

<details>

_Script_

The `ptr_to_option` function is a port of the [`as_mut` method on pointers] that
we used in the previous slide.

_Instructions_

- Add a safety section to the docstring
- Add a safety comment in the body of the function
- Click through to the std lib docs for `as_mut`

[`as_mut` method on pointers]: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut

</details>

---

# Warm up: unsafe traits

```rust
# TODO: finish with full paths

struct StatusIndicator(AtomicIsize32)

// impl Send for Indicator {};
// impl Sync for Indicator {};
```

<details>

_Script_

`Send` and `Sync` are so-called _unsafe traits_ relating to concurrency. What
does it mean to implement an unsafe trait?

When you implement them for your types, you're assuming responsibility for
upholding Rust's safety guarantees. When a library author defines an unsafe
trait, it means that they're providing an interface that carries risks that the
compiler cannot protect the implementer from.

The burden is on the implementer to ensure that the trait's safety
pre-conditions are satisfied.

Here, we have a newtype wrapping an atomic type. Atomic types--types which are
updated by the computer without the possibility of different threads seeing
different values--are both `Send` and `Sync`. As we're not adding any additional
behavior, we can be confident that `StatusIndicator` it follows that we have a
green light to implement those types ourselves.

So let's go ahead and implement them.

The syntax for implementing `Send` and `Sync` is quite minimal. They're marker
traits, so they don't have any methods. The vast majority of the time taken to
implement them is spent ensuring that your implementation follows Rust's rules.

</details>

---

We know that writing code without the guarantees that Rust provides ...

> “Use-after-free (UAF), integer overflows, and out of bounds (OOB) reads/writes
> comprise 90% of vulnerabilities with OOB being the most common.”
>
> --— **Jeff Vander Stoep and Chong Zang**, Google.
> "[Queue the Hardening Enhancements]"

... so why is `unsafe` part of the language?

[Queue the Hardening Enhancements]: https://security.googleblog.com/2019/05/queue-hardening-enhancements.html

---

# Why unsafe

- Necessity
- Usefulness

<details>

Necessity:

- interacting with _uninitialized memory_
- accessing CPU and compiler _intrinsics_
- interacting with external systems FFI (foreign function interface), including
  other programming language runtimes such as Java and its JNI (Java Native
  Interface)
- interacting with the host platform
- writing a memory allocator
- implementing concurrency primitives
- implementing data structures that Rust's borrow checker is unable to reason
  about, such as graphs that may have cycles and intrusive data structures

Usefulness:

- your code can go faster

</details>

---

# If you are optimizing for speed, then consider alternatives to unsafe first

Unsafe code

- is difficult to write
- risks introducing safety, security, and stability bugs
- needs more code review

TODO: effort vs performance diagram - unsafe offers diminishing returns

<details>

Using unsafe to improve the code's speed is often at the end of the list of
optimizations.

It's difficult to write correct unsafe code quickly.

Because of its hazards, it's subject to more strenuous review, which slows down
development time.

Therefore, unless it's used in context that could benefit from marginal
increases in performance such as systems that are at high volume and/or high
velocity,

</details>

---

# What is Unsafe Rust?

<!-- TODO: add markup to exclude diagram from translation  -->

```bob
╭───────────────────────────────────────────────────────────╮
│╭─────────────────────────────────────────────────────────╮│
││                                                         ││
││  Safe                                                   ││
││  Rust                                                   ││
││                                                         ││
││                                                         ││
│╰─────────╮                                               ││
│          │                                               ││
│  Unsafe  │                                               ││
│   Rust   │                                               ││
│          ╰───────────────────────────────────────────────╯│
╰───────────────────────────────────────────────────────────╯
```

<details>

_Script_

Unsafe Rust is a superset of the Rust language that you already know.

It enables access to a few primitive unsafe operations, but doesn't disable
anything that you already have, such as a borrow checker or type safety.

As we'll discover, those unsafe operations provide the foundation that the rest
of Rust is built from.

Until then, one thing to remember about the differences between an unsafe
operation and safe operation is where the burden of the proof lies for upholding
Rust's guarantees. The Rust compiler takes responsibility for safe operations,
whereas the programmer is responsible for operations marked unsafe.

We'll be spending lots of the time explaining how to work with that burden of
proof.

</details>

---

# Burden of proof

---

# Memory life cycle

<!-- TODO: check suitability for translation; is it okay to use labels with numbers & a legend? -->
<!-- TODO: check accessibility; should each border be different to make it easier to talk about? -->
<!-- TODO: add markup to exclude diagram from translation  -->

```bob
╭──────╮       ╭──────╮      ╭──────────────────────────────╮ 
│  1   │       │  2   │      │ 3                            │
│      │       │      │      │   ╭───────╮     ╭───────╮    │
│      │       │      │      │   │       │     │       │    │
│      │  -->  │      │  --> │   │       │ --> │       │    │ 
│      │       │      │      │   │  3.1. │     │  3.2. │    │ 
│      │  <--  │      │  <-- │   │       │     │       │    │
│      │       │      │      │   │       │ <-- │       │    │ 
│      │       │      │      │   ╰───────╯     ╰───────╯    │ 
│      │       │      │      │                              │
╰──────╯       ╰──────╯      ╰──────────────────────────────╯
```

### Legend

1. Hardware
2. OS
3. Program
   1. Allocator
   2. Initialized memory

---

# Things that are not unsafe

- panicking
- leaking memory

## Panicking

```rust
fn main() {
    panic!("This crashes, but at least the program is memory safe!");
}
```

<details>

Discuss why.

</details>

## Leaking memory

```rust
fn leak<T>(val: T) {
    Box::new(val).leak()
}

fn main() {
    let a = 1;

    leak(a);
}
```

<details>

Show learners through the example and discuss why leaking memory is not a memory
safety issue.

_Suggested exercise_

Ask learners to draw a line of the lifecycle of the value of `a`. You may need
to discuss that variables on the stack are allocated do not use a memory
allocator.

</details>

---

MaybeUninit

- idea: use `Box::new_uninit()` as a bridge to `MaybeUninit<T>`

---
