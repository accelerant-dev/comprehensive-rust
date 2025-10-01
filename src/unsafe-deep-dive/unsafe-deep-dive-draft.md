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

```rust,editable
fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut _;
    let b: *mut i32 = std::ptr::null_mut();

    println!("{:?}", *a);
    println!("{:?}", b.as_mut());
}
```

<details>

_Instructions_

- Introduce code
  - [Line 3] Creates raw pointer to the `123` by de-referencing the box,
    creating a new reference and casting the new reference as a pointer
  - [Line 4] Creates raw pointer with a NULL value
  - [Line 7] Converts the raw pointer to an Option with `.as_mut()`;
- Compile to reveal the error messages
- Discuss
  - [Line 6] De-referencing a raw pointer
  - [Line 7] Calling a function that's marked as unsafe
- Fix the code and compile again to show the working program
- Add safety comments above the two calls saying that we know that the memory is
  valid
- Discuss the possibility of using a single unsafe block rather than one for
  each line. Mention that we want to cover a single case with an unsafe block to
  avoid masking errors and to make safety comments as specific as possible

</details>

---

# Warm up: defining an unsafe function

```rust,editable
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

    println!("{:?}", unsafe { *a });
    println!("{:?}", ptr_to_option(b));
}
```

<details>

<!-- TODO: finish script -->

_Script_

The `ptr_to_option` function is our own version of the [`as_mut` method][as_mut]
on pointers that we used in the previous slide.

_Instructions_

- Mark `ptr_to_option` as unsafe
- Document safety pre-conditions
  - Refer to the [original's documentation][as_mut]
- Add a safety section to the docstring
- Add a safety comment in the body of the function
- Click through to the std lib [docs for `as_mut`][as_mut]

[as_mut]: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut

</details>

---

# Warm up: unsafe traits

```rust,editable
struct StatusIndicator(std::sync::atomic::AtomicI32);

// impl std::marker::Send for StatusIndicator {}
// impl std::marker::Sync for StatusIndicator {}
```

<details>

Material to cover

- Traits for anyone that may still be somewhat unfamiliar; foster an idea that
  they are sets of requirements
- `Send` and `Sync` traits and their relationship to concurrency, an area where
  safety concerns are prominent
- Marker traits add information to the type system
- `unsafe` traits have requirements with safety consequences

_Instructions_

- Follow the script below

_Script_

`Send` and `Sync` are so-called _unsafe traits_ relating to concurrency.

What does it mean to implement an unsafe trait?

To answer that, we first need to ensure that everyone agrees what a trait is.

[Asking audience] What is a trait?

[Gather ideas and discuss]

Traits are normally described as an _interface_ [Java/Go] or _protocol_
[Python], or perhaps even a type class if you have a Haskell background.

These descriptions focus on the methods that the trait provides. They focus on
the shared behavior.

I would like to suggest a slightly different perspective that places less
emphasis on behavior. What about, "traits as sets of requirements"?

Allow me to explain what I mean...

When you define a trait, you're specifying a set of conditions that types must
satisfy before they're able to implement them.

And when you implement a trait, you're providing an assurance to the type system
that the type meets the trait's requirements.

[PAUSE]

For example, let's consider the [`Eq`] trait. When you learned Rust, you may
have been slightly confused as to why there's a `PartialEq` trait that is used
for the equality operator, but there is also an additional trait `Eq` that
provides no new methods.

After some time, you discovered that although it provides no new methods, `Eq`
does provide new semantics.

For a type to implement `Eq`, every value of that type must be equal to itself.
Floating point NaN values do not uphold this requirement. Therefore, `f32` and
`f64` are not `Eq` types.

Formally speaking, `Eq` types are said to upload the [_reflexive relation_].

[PAUSE]

Thinking of traits this way makes it easier to understand what the purpose of a
_marker trait_ is.

Although it doesn't provide any new methods, marker traits provide information
to the type system. They confirm that types that implement them satisfy the
requirements.

Thinking of traits this way also makes it easier to understand an unsafe trait
is.

An unsafe trait is a trait that is special because if you fail to meet its
requirements, you will be violating Rust's safety guarantees.

[PAUSE]

[Asking audience] Any questions at this point? It's okay to disagree with me
about any of this -- I would be interested to hear any thoughts.

[PAUSE]

Okay, now that we've established what a trait is, let's take a closer look at
the two traits here, `Send` and `Sync`.

The first thing that you might notice is that they're defined within the
`std::marker` module of the standard library. That is an indication that `Send`
and `Sync` are used to enrich the type system.

Let's reiterate. When you implement either of these traitsfor your types, you're
assuming responsibility for upholding Rust's safety guarantees. When a library
author defines an unsafe trait, it means that they're providing an interface
that carries risks that the compiler cannot protect the implementer from.

The burden is on the implementer to ensure that the trait's safety
pre-conditions are satisfied.

Here, we have a newtype wrapping an atomic type. Atomic types--types which are
updated by the computer without the possibility of different threads seeing
different values--are both `Send` and `Sync`. As we're not adding any additional
behavior, we can be confident that `StatusIndicator` it follows that we have a
green light to implement those types ourselves.

So let's go ahead and implement them. [Uncomment lines; compile to show the
error; add the unsafe keyword; re-compile]

The syntax for implementing `Send` and `Sync` is quite minimal. They're marker
traits, so they don't have any methods. The vast majority of the time taken to
implement them is spent ensuring that your implementation follows Rust's rules.

[_reflexive relation_]: https://en.wikipedia.org/wiki/Reflexive_relation
[`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html

<!-- TODO: move the content below to later in the course; this is suppose to be a warm up -->

_Instructions (cont.)_

> _Note:_
>
> Avoid spending too much time here. The aim is to inform the audience that
> these traits and types have quirks.

- Open the standard library's documentation for
  - [`Send`][send-docs]
    - Discussion points
      - "Types that can be transferred across thread boundaries [by copying the
        bytes exactly as they currently are in memory]"
      - "This trait is automatically implemented when the compiler determines
        it’s appropriate." - similar to `Sized`
      - Not notation, i.e. `impl !Send for Args`
    - Raise question: What are the safety pre-conditions for `Send`?
  - [`Sync`][sync-docs]
    - Contrast with `Send`: `Sync` primarily relates to sharing references to
      values, whereas `Send` primarily relates to sharing values themselves
    - Sync has some complex semantics, esp. with
      - Confusion between `&T` and `&mut T`
      - Interior mutability
      - Pointers guaranteed to be non-NULL, i.e. "`impl<T> !Sync for NonNull<T>`
        NonNull pointers are not Sync because the data they reference may be
        aliased."
    - Raise question: What are the safety pre-conditions for `Sync`?
  - [atomic operations][atomic-docs].
    - If your audience has a C++ background, mention that the semantics of Rust
      and C++ differ.
    - Sentences to highlight and points to emphasize
      - **Portability** section
        - Atomic operations may be emulated
        - "Atomic types and operations are not guaranteed to be wait-free."
      - **Atomic accesses to read-only memory** section
        - "In general, all atomic accesses on read-only memory are undefined
          behavior."

[atomic-docs]: https://doc.rust-lang.org/std/sync/atomic/index.html
[send-docs]: https://doc.rust-lang.org/std/marker/trait.Send.html
[sync-docs]: https://doc.rust-lang.org/std/marker/trait.Sync.html

</details>

---

# Why unsafe

> “Use-after-free (UAF), integer overflows, and out of bounds (OOB) reads/writes
> comprise 90% of vulnerabilities with OOB being the most common.”
>
> &mdash; **Jeff Vander Stoep and Chong Zang**, Google.
> "[Queue the Hardening Enhancements]"

We know that writing code without the guarantees that Rust provides is
dangerous, so why is `unsafe` part of the language?

[Queue the Hardening Enhancements]: https://security.googleblog.com/2019/05/queue-hardening-enhancements.html

---

# Why unsafe (cont.)

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

# Examples of safety comments

---

# Examples of safety comments : Re-state common knowledge

From [`std::ptr::NonNull.from_ref`]:

```rust
pub const fn from_ref(r: &T) -> Self {
    // SAFETY: A reference cannot be null.
    unsafe { NonNull { pointer: r as *const T } }
}
```

<details>

_Script_

Even though it can feel redundant and obvious, it's still useful to re-state
safety rationale that is common knowledge in the Rust community.

Here's an example from the standard library. We have a method that takes a
shared reference and converts it to a pointer.

The authors take the time to re-state one of the first rules of references, that
they cannot be null.

[`std::ptr::NonNull.from_ref`]: https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.from_ref

</details>

---
