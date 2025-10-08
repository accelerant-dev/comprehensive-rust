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

<!-- TODO: pull more content from the existing draft material into the start of this section -->

# Whole unsafe deep dive course draft

## Day 1: Morning

---

# Welcome!

This deep dive aims to enable you to work productively with Unsafe Rust.

By the end, you'll be able to:

- use APIs with `unsafe`
- build APIs with `unsafe`
- review code that includes `unsafe`

<details>

Achieving that aim requires:

- Deepening your knowledge
  - a mental model of how memory works
  - what the `unsafe` keyword means
  - a shared vocabulary
  - common patterns
  - expectations for code that uses `unsafe`

- Practice working with unsafe
  - reading and writing both code and documentation

- Review code
  - the confidence to self-review easy cases
  - the knowledge to detect difficult cases

</details>

---

# Your aims

What are your goals for this class?

<details>

- Signal when the content will be offered the course
- Note down any points that differ from the course material

</details>

---

# Defining Unsafe Rust

<!-- mdbook-xgettext: skip -->

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

- Unsafe Rust is a superset of Safe Rust
- Unsafe Rust adds extra capabilities, including
  - Dereference raw pointers
  - Call functions marked as unsafe
- Those capabilities are referred to as _unsafe operations_
- Unsafe operations provide the foundation that the rest of safe Rust is built
  on
- Rust still applies most of the rules, including type safety and borrow
  checking

</details>

---

# The unsafe keyword has two roles

- Creating APIs with safety considerations
- Using APIs with safety considerations

<details>

- Creating APIs with safety considerations
  - Functions marked as unsafe, i.e. `unsafe fn`
  - Traits marked as unsafe, i.e. `unsafe trait`
- Using APIs with safety considerations
  - Unsafe blocks: `unsafe { ... }`
  - Unsafe trait implementations: `unsafe impl { ... }`
- Note that the keyword "unsafe" does not automatically imply a problem. It
  implies that improper use of the code is unsafe.
- Consider opening the [unsafe keyword] documentation
  - Briefly mention (explain that we'll cover them this morning)
    - (Top of page ) "...existence of **contracts** the compiler can’t check..."
    - Undefined behavior
    - Soundness
    - "...it is now up to you to ensure soundness..."

[unsafe keyword]: https://doc.rust-lang.org/stable/std/keyword.unsafe.html

</details>

---

# Warm up

4 examples:

- using an unsafe block
- defining an unsafe function
- implementing an unsafe trait
- defining an unsafe trait

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

- Code walkthrough
  - Confirm understanding
    - `Box`
    - `*mut i32`
  - [Line 3] Creates raw pointer to the `123` by de-referencing the box,
    creating a new reference and casting the new reference as a pointer
  - [Line 4] Creates raw pointer with a NULL value
  - [Line 7] Converts the raw pointer to an Option with
    [`.as_mut()`][ptr-as_mut];
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

[ptr-as_mut]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_mut

_Suggested Solution_

```rust
fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut _;
    let b: *mut i32 = std::ptr::null_mut();

    // SAFETY: `a` refers to an i32 and all values are valid.
    println!("{:?}", unsafe { *a });

    // SAFETY: `b` is a null pointer, which can always be converted to None.
    println!("{:?}", unsafe { b.as_mut() });
}
```

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

    println!("{:?}", ptr_to_option(a));
    println!("{:?}", ptr_to_option(b));
}
```

<details>

_Instructions_

- Code walkthrough
  - Mention that `ptr_to_option` function is our own version of the
    [`as_mut` method][ptr-as_mut] on pointers that we used in the previous
    slide.
  - Ask for a quick review of the code
    - If someone mentions the lack of the `unsafe` keyword, request that they
      explain why unsafe is necessary
- Compile the code
- Click through to the [documentation original method][ptr-as_mut]
  - Note that safety rules relating to pointer semantics are subtle
- Mark `ptr_to_option` and calls as unsafe
- Add safety pre-conditions (suggested solution below)
  - Add a safety section to the docstring
  - Add a safety comment in the body of the function

_Notes_

- Learners may notice the auto-deref behavior

_Suggested Solution_

```rust
/// Convert a pointer to an `Option<T>`
///
/// Returns `None` when `val` is null, otherwise wraps `val` in `Some`.
///
/// # Safety
///
/// When calling this method, ensure that either the pointer is null or
/// the pointer is convertible to a reference.
///
/// Pointers are convertable to a reference when they are guaranteed to
/// point to a valid instance of `T`, are correctly aligned, obey Rust's
/// aliasing rules and are "dereferenceable" as described in the [documentation of `std::ptr`].
///
/// [documentation of `std::ptr`]: https://doc.rust-lang.org/std/ptr/index.html#safety
unsafe fn ptr_to_option<'a, T>(val: *mut T) -> Option<&'a mut T> {
    if val.is_null() { None } else { unsafe { Some(&mut *val) } }
}

fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut _;
    let b: *mut i32 = std::ptr::null_mut();

    // SAFETY: `a` refers to an i32 and all values are valid.
    println!("{:?}", unsafe { ptr_to_option(a) });

    // SAFETY: `b` is a null pointer, which can always be converted to None.
    println!("{:?}", unsafe { ptr_to_option(b) });
}
```

</details>

---

# Warm up: using an unsafe trait

```rust,editable
struct StatusIndicator(std::sync::atomic::AtomicI32);

// impl std::marker::Send for StatusIndicator {}
// impl std::marker::Sync for StatusIndicator {}
```

<details>

- Confirm understanding
  - Explain traits for anyone that may still be somewhat unfamiliar; foster an
    idea that they are sets of requirements
- `Send` and `Sync` traits and their relationship to concurrency, an area where
  safety concerns are prominent
- Marker traits add information to the type system
- `unsafe` traits have requirements with safety consequences

</details>

---

# Warm up: defining an unsafe trait

From the [`std::marker`]:

```rust
pub unsafe auto trait Send {}
pub unsafe auto trait Sync {}
```

<details>

- Unsafe traits are typically marker traits
- The safety concerns can't directly be expressed in code and rely on human
  -to-human communication.

</details>

[`std::marker`]: https://doc.rust-lang.org/std/marker/index.html

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

# Burden of proof

- The compiler is responsible for Safe Rust
- The programmer is responsible for Unsafe Rust

---

# Safety pre-conditions

A safety pre-condition is a condition on a type that must be upheld when a value
type constructed.

```rust,editable
fn main() {
    let b: *mut i32 = std::ptr::null_mut();
    println!("{:?}", b.as_mut());
}
```

[`std::ptr.as_mut()`][ptr-as_mut]

<details>

Compile broken code and then follow the documentation to confirm that an unsafe
block can be safety added to enable to code to function without triggering
undefined behavior.

_Detailed Instructions_

- Compile the code to generate the error
- Explain the notes provided by rustc
  - "note: consult the function's documentation for information on how to avoid
    undefined behavior"
- Consult [the documentation][ptr-as_mut]
  - Highlight Safety section
    - "When calling this method, you have to ensure that either the pointer
      [is null] or the pointer is [convertible to a refer]ence."
  - Questions to raise
    - who is the audience for the Safety section in the doc comment?
    - how does a doc comment differ from in-line comments?
    - why does a null pointer always satisfy the safety condition?
- Click the "[convertible to a reference]" hyperlink to the "Pointer to
  reference conversion"
  - Track down the rules for converting a pointer to a reference, aka is
    "_deferencerable_"
  - Emphasize that many types have complicated semantics
  - Consider the implications of this excerpt (Rust 1.90.0) "You must enforce
    Rust’s aliasing rules. The exact aliasing rules are not decided yet, ..."

[is null]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.is_null-1
[convertible to a reference]: https://doc.rust-lang.org/stable/std/ptr/index.html#pointer-to-reference-conversion

_Aim of slide_

Introduce the notion of pre-conditions by looking at pre-conditions that have
already been written down.

</details>

---

# Burden of proof (expanded)

- The compiler is responsible for Safe Rust
- The programmer is responsible for Unsafe Rust
  - When creating abstractions that require an unsafe block, i.e. an
    `unsafe fn`, safety pre-conditions must be documented

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

# Rules of the game

Learning objectives

- mental model
- terminology

<details>

The goal of this section is to introduce the terms "undefined behavior",
"sound", "unsound", and the mental framework behind soundness of Rust code that
contains unsafe.

</details>

---

# Rules of the game : defining undefined behavior

- what it is
- when it occurs
- why it is dangerous
  - breaks tooling
    - compiler optimization
    - code analysis, incl. model checkers and formal reasoning
- why it is difficult

<details>

We've talked a fair amount so far in the class about the term _undefined
behaviour_. The next few slides are dedicated to explaining what it is and why
it's a problem.

But what does it actually mean?

While it may sound like exaggeration, _undefined behahavior_ means that anything
can happen at runtime. Anything at all.

Let's look at why.

Compilers use rules to decide what to optimize.

Compilers attempt to optimize in two directions. First, they try to make the
code go faster. Secondly, they try to make the generated code shorter. Those two
directions are sometimes at odds, but let's ignore that for now. We can talk
about Pareto-optimal frontiers over lunch.

Let's say there are two ways to compile your code that produce the same result.
Other things being equal the compiler will chose the faster code.

For example, in your source code, you might have tried to detect whether a
number is even or odd by diving by 2 and checking the remainder with the modulus
operation. Compiler engineers have seen this trick too! They will ignore that
expression and replace it with a comparison with the least significant bit. For
even numbers, the last bit of an integer is zero. For odd numbers, it's one.

Integer division is quite slow within a computer and compiler will often seek to
replace division with something that's functionally equivalent.

More broadly, compilers treat your source code as intent rather than
prescriptive.

</details>

---

# Rules of the game : defining undefined behavior : what it is

Effects that your compiler assumes are impossible to occur:

- dereferencing a null pointer
- accessing an array out of bounds
- signed integer overflow
- data races
- reading from uninitialized memory

<details>

Each of these is a case of undefined behavior. Your compiler will assume they
don't occur, and will optimize accordingly.

- **dereferencing a null pointer**, because pointers are never null
- **accessing an array out of bounds**, because program would never attempt to
  access the 11th element of an array that contains 10
- **signed integer overflow**, because mathematical operations will always stay
  within the bounds that the type can represent
- **data races on shared memory**, because concurrency bugs do not occur
- **reading from uninitialized memory**, because there's no guarantees about
  what's actually sitting in RAM at that address

These cases are each quite interesting, but we'll be spending some extra time
shortly on uninitialized memory. It's a topic that deserves its own treatment.

</details>

---

# Rules of the game : defining undefined behavior : why it's dangerous

The compiler might produce code that

- works as expected
- changes its behavior depending on the optimization level
- crashes at runtime
- corrupts data
- leaks data
- has important code paths deleted

Therefore

- once your program has triggered undefined behavior, it is impossible to reason
  about

<details>

<!-- TODO: finish -->

Optimization levels are particularly relevant because most testing of generated
Rust occurs during `--debug` builds.

Before then, let's look at what I mean by suggesting that important code paths
may be deleted.

</details>

---

# Rules of the game : defining undefined behavior : why it's dangerous

## Dead code elimination (DCE) - part 1

```rust
pub fn rand_int() -> i32 {
    if true { 0 } else { 1 }
}
```

<details>

To understand the risks, it can be useful to refresh your understanding of what
a compiler is and what it does.

A compiler takes your source code, a sequence of bits that can be interpreted by
humans, and converts it to a different sequence of bits which can be interpreted
by a CPU. But that's not all it does.

Take a look at the code up screen. What would we expect the compiler to do with
it?

The compiler could mechanically translate the if block into the executable.

Or, it could apply the rules that it knows about if blocks and the value of
`true` to deduce that the block containing the number `1` will never be reached.
And it could delete it.

_Key points_

- if you were a compiler, how would you compile this code?
- compilers delete code that's unnecessary
- (recommendation) open code in the [playground][playground-dce] or
  [compiler explorer][godbolt-dce] and

[playground-dce]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a946c1fedf292d64c61b4e30b88c7790
[godbolt-dce]: https://rust.godbolt.org/z/7qTK57K1j

</details>

---

# Rules of the game : defining undefined behavior : why it's dangerous

## Dead code elimination (DCE) - part 2

<!-- TODO: simplify code example -->

```rust
#[derive(Debug)]
pub struct Token(String);

#[derive(Debug)]
pub struct ValidToken<'a>(&'a Token);

#[derive(Debug)]
pub struct SecurityViolation<'a>(&'a Token);

/// Bless `Token` as a `ValidToken`
///
/// When validation fails, return `SecurityViolation` as an error.
pub fn validate(token: &Token) -> Result<ValidToken, SecurityViolation> {
    let data: &String = &(*token).0;

    // SAFETY: ensure that reference is valid
    if (token as *const Token).is_null() {
        return Err(SecurityViolation(token));
    }

    // validate token
    if data.contains("password") {
        return Err(SecurityViolation(token));
    }

    Ok(ValidToken(token))
}
```

<details>

- what about this code?
  - de-referencing a null pointer is undefined behavior, therefore it never
    occurs, therefore the check is deleted
- this is a simplified version of a case that affected the Linux kernel
- _Note:_ Rust's ownership semantics makes this error significantly more
  difficult to obfuscate than C

</details>

---

# Rules of the game : defining undefined behavior : why it's difficult

<!-- TODO: better wording -->

Difficulty comes from (at least) three sources

- difficult to detect
- difficult semantics
- difficult to trigger

<details>

Many data types impose specific rules about how they are implemented.

For example, Rust's references must never have the value zero. This is

- difficult to detect
  - many soundness holes exist for a long time without anyone noticing
- difficult semantics
  - it can be difficult know whether behavior is actually unsound, as
    - some types have very specific semantics
    - specifications are subject to interpretation and can also contain errors
- replicate target environment
  - it's sometimes difficult to replicate target environment
    - some bugs are triggered within a specific context that might be difficult
      to replicate under test

</details>

---

# Recap - sound code

<details>

- when code is sound, it is constructed in a way that's safe

- soundness is a property that describes being valid by definition
- c.f. logic

</details>

---

# Rules of the game : 3 functions

```rust,editable
/// Replace contents of `container` with items produced by `generator`
pub fn fill<T>(container: &mut [T], mut generator: impl FnMut(usize) -> T) {
    for (i, item) in container.iter_mut().enumerate() {
        *item = generator(i);
    }
}
```

```rust,editable
/// Replace `count` items of `container` with items produced by `generator`
///
/// The updated `container` will contain initialized memory, allowing
/// callers to `assume_init()`.
pub fn partial_fill_maybe_uninit<T>(
    container: &mut [std::mem::MaybeUninit<T>],
    count: usize,
    mut generator: impl FnMut(usize) -> T,
) {
    for i in 0..count {
        let elem = unsafe { container.get_unchecked_mut(i) };
        let item = generator(i);
        elem.write(item);
    }
}
```

```rust,editable
/// Replace contents of `container` with items produced by `generator`
///
/// The updated `container` will contain initialized memory, allowing
/// callers to `assume_init()`.
pub unsafe fn partial_fill_maybe_uninit_unchecked<T>(
    container: &mut [std::mem::MaybeUninit<T>],
    let count: usize,
    mut generator: impl FnMut(usize) -> T,
) {
    let count = count.min(container.len()),
    for i in 0..count {
        let elem = unsafe { container.get_unchecked_mut(i) };
        let item = generator(i);
        elem.write(item);
    }
}
```

<details>

- `fill`
  - The compiler knows that Rust's iterators won't go out of bounds
  - This is the normal Rust language that you all use.
  - This is how most Rust code should look.

- `partial_fill_maybe_uninit`
  - Contains a "silent" unsafe block, i.e. including one within a function
    that's being marked unsafe
  - Triggers UB when `count > buffer.len()`
  - Make improvements
    - Document preconditions and mark it unsafe
      - The function does not need to satisfy the preconditions of the unsafe
        block itself
      - Marking the function as unsafe shifts this responsibility to the caller
      - The danger is clearly marked
        - For humans, with a precondition
        - For the compiler, with the unsafe keyword
    - Consider re-writing to use an API that's unable to misused

- `partial_fill_maybe_uninit_unchecked`
  - "Crying wolf" example - no need for to be marked as unsafe
  - Make improvements:
    - Add safety comment, i.e.
    - `// SAFETY: Max count is container.len()\ so i is in-bounds`

</details>

---

# Recap : Unsafe keyword

| Context                                 | Function of keyword                              | Docs Required                           | Docs Location                  |
| --------------------------------------- | ------------------------------------------------ | --------------------------------------- | ------------------------------ |
| `unsafe fn`, `unsafe trait`             | declares safety pre-conditions exist             | What pre-conditions exist               | Public API docs                |
| `unsafe { ... }`, `unsafe impl { ... }` | confirms that safety preconditions are satisfied | How they are guaranteed to be satisfied | SAFETY: comment in source code |

<details>

The keyword has 2 roles:

- When building abstractions, the `unsafe` keyword signals that safety
  pre-conditions exist that the compiler cannot verify
- When using abstractions, the `unsafe` keyword confirms that the pre-conditions
  are satisfied

Documentation must exist that describes what the safety pre-conditions are

</details>

---

# Recap

<!-- TODO: finish -->

- Rust code should be sound code, and sound code must be memory safe code
- `unsafe` keyword shifts responsibility from the compiler to the programmer
- `unsafe` requires human review

<details>

- soundness implies safety, but safety does not imply soundness
  - `Sound ⊂ Safe`
    - `Sound → Safe`, i.e. being sound implies being safe
    - but, `¬(Safe → Sound)`, i.e. being safe does not imply being sound;
      equivalently `∃x: Safe(x) ∧ ¬Sound(x)`
- "safe", "unsafe" and precondition documentation/comments are compile-time
  promises
- "sound" and "unsound" are judgments about whether the code follows the
  agreed-upon rules about marking things "unsafe" and documenting preconditions.
- Unsafe operations and precondition comments are not checked automatically.

---

<!-- TODO: delete - ideas only -->

# Safety comments

perhaps mention in earlier code that a later unsafe block depends on its state.
