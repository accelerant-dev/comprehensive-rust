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

- use unsafe APIs
- build unsafe APIs
- review unsafe code

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

We'll be using a spiral model of teaching. We'll introduce the same topic
multiple times with increasing depth.

</details>

---

# Introductions

- Who are you?
- What are you working on?
- What are your goals for this class?

<details>

- Note down any points that differ from the course material

</details>

---

# Agenda

<!-- TODO: create before final publication -->

## Day 1

- ...

## Day 2

- ...

---

# Defining Unsafe Rust

Only values in this state **3.2.** are correctly initialized. When `C > len`, it
allows the vector to contain items in state **3.1.**.

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
  - When used in this role, the unsafe keyword indicates that users of the API
    will need to be careful. It's important for the creator of the API to
    communicate what care needs to be taken.
- Using APIs with safety considerations
  - Unsafe blocks: `unsafe { ... }`
  - Unsafe trait implementations: `unsafe impl { ... }`
  - When used in this role, the unsafe keyword means that the author has been
    careful. They have verified that the code is safe and is providing an
    assurance to others.
- Note that the keyword "unsafe" may have a subtly different meaning than what
  some people assume.
  - In principle, the code is safe.
  - However, improper usage is highly dangerous.
  - And it's impossible for the compiler to verify that the usage is correct.
- Open the [unsafe keyword] documentation and highlight some key terms and
  phrases. Mention that we'll be explaining these terms as we go through the
  morning.
  - (Top of page ) "...existence of **contracts** the compiler can’t check..."
  - Undefined behavior
  - Soundness
  - "...it is now up to you to ensure soundness..."

[unsafe keyword]: https://doc.rust-lang.org/stable/std/keyword.unsafe.html

</details>

---

# Warm up

4 examples follow:

- using an unsafe block (`unsafe { ... }`)
- defining an unsafe function (`unsafe fn`)
- implementing an unsafe trait (`unsafe impl { ... }`)
- defining an unsafe trait (`unsafe trait`)

<details>

- These examples are intended to:
  - Introduce the syntax
  - Provide a general idea of how unsafe works
  - Activate learners' brains
  - Reveal knowledge gaps
- Encourage comments, but defer detailed explanations
- Ask learners to write down questions as they arise

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
/// Convert a nullable pointer to an `Option<T>`.
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

- Step through code
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
- Add safety preconditions (suggested solution below)
  - Add a safety section to the docstring
  - Add a safety comment in the body of the function

_Suggested Solution_

```rust
/// Convert a nullable pointer to an `Option<T>`.
///
/// Returns `None` when `val` is null, otherwise wraps `val` in `Some`.
///
/// # Safety
///
/// When calling this method, ensure that either the pointer is null or
/// the pointer is convertible to a reference.
///
/// Pointers are convertible to a reference when they are guaranteed to
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

- Traits are sets of requirements
- The compiler can verify the requirements for most traits
  - To verify that type `T` implements `std::fmt::Display` , the compiler looks
    for a matching `fn fmt()` method
- More difficult for unsafe traits; potential safety concerns cannot be
  expressed in code
- `Send` and `Sync` are marker traits
- Marker traits are used add information to the type system that cannot be
  expressed directly in code
- Most marker traits are used to enable the type system to protect code from
  logical errors
  - Consider `std::cmp::Eq`: it exists to prevent floating point values from
    being used where they might cause problems
- `Send` and `Sync` are relate to concurrency, an area where safety concerns are
  prominent
- Adding `unsafe` to a keyword trait signals that implementing it incorrectly
  has safety consequences

</details>

---

# Warm up: defining an unsafe trait

```rust,editable
/// Implementors are represented with 32 bits on all platforms.
pub trait Width32 {}

impl Width32 for i32 {}
impl Width32 for f32 {}
```

<details>

- Describe a scenario where a library author wishes to mark some types as being
  a specific number of bits wide
- Discuss why the current code is insufficient - memory safety implications
- Add the unsafe keywords
- Add safety comment

_Suggested Solution_

```rust
/// Implementors are represented with 32 bits on all platforms.
///
/// # Safety
///
/// Implementing this trait for types that do not use 32 bits is a memory
/// safety violation.
pub trait Width32 {}

impl Width32 for i32 {}
impl Width32 for f32 {}
```

_Bonus Teaser_

The code won't above work as-is. Using compile-time assertions, it's possible to
tell the compiler that `T` is a specific size:

```rust
pub unsafe trait Width32: Copy {
    // Compile-time assertion to prove to the compiler that T's size equals 4
    const SIZE: () = assert!(std::mem::size_of::<Self>() == 4);
}

unsafe impl Width32 for i32 {}
unsafe impl Width32 for f32 {}

fn view<T: Width32>(val: T) {
    let _ = T::SIZE; // Trigger the assertion

    let bytes: u32 = unsafe { std::mem::transmute_copy(&val) };
    println!("{:x}", bytes);
}

fn main() {
    let a = 123;
    view(a)
}
```

</details>

---

# Working with Unsafe Rust

<!-- TODO: before publication, check that the titles have remained the same -->

- Unsafe is dangerous
- Unsafe is (sometimes) necessary
- Unsafe is (potentially) useful
- Unsafe shifts the burden of proof
- Unsafe requires a stronger development workflow

<details>

- The next few slides will look at unsafe from a different perspective
- Rather than looking at the syntax, we'll think about:
  - The motivations for using unsafe
  - The hazards that unsafe introduces
  - What those hazards imply for teams writing software using it

</details>

---

# Unsafe is dangerous

> “Use-after-free (UAF), integer overflows, and out of bounds (OOB) reads/writes
> comprise 90% of vulnerabilities with OOB being the most common.”
>
> &mdash; **Jeff Vander Stoep and Chong Zang**, Google.
> "[Queue the Hardening Enhancements]"

We know that writing code without the guarantees that Rust provides is
dangerous, so why is `unsafe` part of the language?

[Queue the Hardening Enhancements]: https://security.googleblog.com/2019/05/queue-hardening-enhancements.html

---

# Unsafe is necessary

Unsafe is sometimes necessary:

- working with the host system
  - CPU instructions
  - compiler intrinsics
  - OS syscalls
- working with memory that is not managed by Rust
  - FFI
  - interacting with _uninitialized memory_
  - writing a memory allocator
- implementing concurrency primitives and advanced data structures
- writing a programming language (Safe Rust is written using Unsafe Rust)

# Unsafe is useful

Unsafe is sometimes useful:

- your code can go faster!

<details>

- If you are optimizing for speed, then consider alternatives to unsafe first.
- Staying in safe Rust means that you can focus on algorithm, data structures
  and logic errors.
- Let compiler engineers worry about micro-optimizations.

</details>

# Unsafe is difficult

Unsafe code

- is difficult to write
- is difficult to maintain
- risks introducing safety, security, and stability bugs, and
- needs more code review

<details>

It's difficult to write correct unsafe code quickly.

Because of its hazards, it's subject to more strenuous review, which slows down
development time.

</details>

---

# Unsafe shifts the burden of proof

- The compiler is responsible for Safe Rust
- The programmer is responsible for Unsafe Rust

<detail>

- The unsafe keyword shifts responsibility for maintaining memory safety from
  the compiler to programmers
- The unsafe keyword is a marker for _safety preconditions_ that the compiler
  cannot check
- Safety preconditions are conditions that must be upheld to keep the code safe;
  they will be explained in much more depth shortly

</detail>

---

# Unsafe requires a stronger development workflow

- Step 1: create software
  - Check that pre-conditions are satisfied
- Step 2: code review
  - Self-review -> reviewer -> unsafe Rust expert (when needed)

<details>

- The unsafe keyword places more responsibility on the programmer, therefore it
  requires a stronger development workflow.
- But there's no such thing as "the programmer". Programmers work in teams.
- This class assumes a specific software development workflow where code review
  is mandatory, and where the author and primary reviewer have access to an
  unsafe Rust expert.
- There are only a few unsafe Rust experts, and they are very busy, so we need
  to optimally use their time.
- The author and primary reviewer will verify simple unsafe Rust code
  themselves, and punt to an unsafe expert when necessary.

</details>

---

# Object life cycle

- Memory model
- Initialization
- `std::mem::MaybeUninit<T>`

<detail>

- We are working towards understanding _safety preconditions_
- Safety preconditions are conditions that need to be satisfied before Unsafe
  Rust is entered so that it follow's Rust's rules
- To get there, we need to take a slight detour towards understanding a little
  bit more about how a chunk of memory becomes a valid variable
- The section uses the term "object" to mean something closer to value or
  variable, rather than the sense implied by object-oriented programming
  languages

</detail>

---

# Uninitialized memory workflow

```rust
use std::mem::MaybeUninit;

fn main() {
    // Step 1: create MaybeUninit
    let mut uninit = MaybeUninit::uninit();

    // Step 2: write a valid value to the memory
    uninit.write(1);

    // Step 3: inform the type system that the memory location is valid
    let init = unsafe { uninit.assume_init() };

    println!("{init}");
}
```

<details>

- General workflow
  1. Create MaybeUninit
  2. Write a value
  3. Confirm unsafe

- Things to avoid
  - Reading from
  - Calling `.write()` twice memory leaks (things )

- Create MaybeUninit -> write value -> `assume_init()`
- `write` is safe, but it still has a precondition,
  - "This overwrites any previous value without dropping it, so be careful not
    to use this twice unless you want to skip running the destructor."

- A note around safety
  - Creating MaybeUninit is safe
  - Writing a value to it is safe

-
  - Reading from MaybeUninit requires `unsafe`
  - Confirming that the memory is now initialized with `assume_init(` requires
    `unsafe`

</details>

---

# Safety preconditions

- Definition
- Introductory example
- Complex example

<details>

- Safety preconditions prevent memory safety problems within `unsafe` blocks.
- First example is another look at some of the code from the warm up
- Second example is new, and will

</details>

---

# Safety preconditions

What are the safety pre-conditions of these three functions?

```rust
pub unsafe fn deref(x: u8) -> bool {
    unsafe { std::mem::transmute(x) }
}

pub unsafe fn u8_to_bool_unchecked(x: u8) -> bool {
    unsafe { std::mem::transmute(x) }
}
```

<details>

- `u8_to_bool_unchecked`:

</details>

---

# Safety preconditions

What are the conditions that must be upheld when `b.as_mut()` is called?

```rust,editable
fn main() {
    let b: *mut i32 = std::ptr::null_mut();
    println!("{:?}", b.as_mut());
}
```

[`std::ptr.as_mut()`][ptr-as_mut]

<details>

Attempt to compile broken code and then follow the documentation to confirm that
an unsafe block can be correctly added to enable to code to function without
triggering undefined behavior.

_Detailed Instructions_

- Compile the code, demonstrate the compilation error
- Explain the notes provided by rustc
  - "note: consult the function's documentation for information on how to avoid
    undefined behavior"
- Consult [the documentation][ptr-as_mut]
  - Highlight Safety section
    - "When calling this method, you have to ensure that either the pointer
      [is null] or the pointer is [convertible to a reference]."
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

Introduce the notion of preconditions by looking at preconditions that have
already been written down.

</details>

---

# Introducing safety preconditions

Let's review this code together:

```rust,editable
/// Create a new `Vec<T>` with capacity `C` and length `len` containing the
/// default value of `T`.
pub fn new_filled_container<T: Default, const C: usize>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(C);
    unsafe {
        let data: *mut T = v.as_mut_ptr();

        for i in 0..len.min(C) {
            data.add(i).write(T::default());
        }

        v.set_len(len);
    }
    v
}
```

<details>

This example is intended to present a piece of code that uses unsafe APIs from
the standard library. It also reinforces the earlier messages about code review
and preferring alternatives to unsafe.

Discussion

- When `C > len`, the code triggers undefined behavior as the vector has
  uninitialized elements
- Perhaps created a case of premature optimization: ("const generics are really
  fast")

_Possible solution_

We could push responsibility for maintaining `C <= len` to callers by marking
the function as unsafe and adding a safety comment in the docstring.

```rust
/// Create a new `Vec<T>` with capacity `C` and length `len` containing the
/// default value of `T`.
///
/// # Safety
///
/// Ensure that `C <= len`.
unsafe pub fn new_filled_container<T: Default, const C: usize>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(C);
    unsafe {
        let data: *mut T = v.as_mut_ptr();

        for i in 0..len.min(C) {
            data.add(i).write(T::default());
        }

        v.set_len(len);
    }
    v
}
```

_Better solution_

A better idea would be to fix the program so that problems cannot arise by
misusing the API.

```rust,editable
/// Create a new `Vec<T>` with capacity `C` and length `len` containing the
/// default value of `T`.
pub fn new_filled_container<T: Default, const C: usize>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(C);
    let len = len.min(C);
    unsafe {
        let data: *mut T = v.as_mut_ptr();

        for i in 0..len {
            data.add(i).write(T::default());
        }

        v.set_len(len);
    }
    v
}
```

_Even better solution_

An even better idea would be to fix the program so that problems cannot arise by
misusing the API.

_Extra content_

This bug can be detected by miri, which will generate the following error
message:

> unsafe precondition(s) violated: Vec::set_len requires that new_len <=
> capacity()
>
> This indicates a bug in the program. This Undefined Behavior check is
> optional, and cannot be relied on for safety

</details>

# Introducing safety preconditions : Memory life cycle (1)

When is it valid to print out the value of `a` and why?

```rust
fn main() {
    // a ≈ 👻
    println!("1. {a}");

    let a; // 🌱

    println!("2. {a}");

    {
        a = 1; // 🌳
        println!("3. {a}");
        // 🍂
    }

    // a ≈ 👻
    println!("4. {a}");
}
```

## A rough view of memory

```bob
         .-.         👻     🌱 [Uninitialized]
    .---+  -+-.   ~~~>~~~~---*-----.
 .-+           +.                   |
|     Memory     |                  * 🌳 [Initialized]
 '- -       - --'                   | 
                  ~~~<~~~~---*-----'  
                     👻      🍂 [Drop]
```

<details>

- It's important to only access variables that are correctly initialized (🌳).
- Other states are not allowed
  - 👻: Impossible to know what the memory contains ("garbage memory")
  - 🌱: Although the value has been defined, and Rust has provided a space for
    that value, the memory hasn't been given a value yet.
  - 🍂: After a value has been dropped, it is no longer valid to access
- When Rust code interacts with memory from outside, Rust must assume that it is
  in the uninitialized state (👻)
- Safe Rust does not allow you to access memory in the uninitialized state
- The mechanism to convert uninitialized memory to an initialized value is
  called `std::mem::MaybeUninit<T>`

</details>

---

# Introducing safety preconditions : Memory life cycle (2)

Only values in this state 3.2 are valid. When `C > len`, it allows the vector to
contain items in state 3.1.

```bob
╭──────╮       ╭──────╮      ╭──────────────────────────────╮ 
│  1   │       │  2   │      │ 3                            │
│      │       │      │      │   ╭───────╮     ╭───────╮    │
│      │       │      │      │   │       │     │       │    │
│      │  -->  │      │  --> │   │       │ --> │       │    │ 
│      │       │      │      │   │  3.1. │     │  3.2. │    │ 
│      │  <--  │      │  <-- │   │       │     │   🌳  │    │
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

<details>

- Programs written with Safe Rust can only refer to memory that's in state
  **3.2.**
- To refer to memory that is not (yet) in state 3.2., Rust provides
  `std::mem::MaybeUninit`.

</details>

---

# MaybeUninit<T>

<details>

-

</details>

---

# MaybeUninit<T>

```
use std::mem::MaybeUninit;

pub fn new_container<const N: usize>(size: usize) -> Vec<i32> { let mut buffer:
[MaybeUninit<i32>; N] = [const { MaybeUninit::uninit() }; N];

    for i in 0..size.min(N) {
        buffer[i] = MaybeUninit::new(0);
    }

    unsafe {
        let mut vec = Vec::<i32>::with_capacity(N);
        let dst: *mut i32 = vec.as_mut_ptr();
        
        for i in 0..size.min(N) {
            dst.add(i).write(buffer[i].assume_init());
        }
        
        vec.set_len(N);
        vec
    }

}
```

<details>

We can see what's happening in more detail by using a type that's provided by
the standard library for working with "uninitialized memory".

This code is almost equivalent to the previous one, except that it's explicit
with the fact that the memory backing `Vec::with_capacity` is not allowed to be
read from right away.

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

The `unsafe` keyword has 2 roles:

- When building abstractions, the `unsafe` keyword signals that safety
  pre-conditions exist that the compiler cannot verify
- When using abstractions, the `unsafe` keyword confirms that the pre-conditions
  are satisfied

Documentation must exist that describes what the safety pre-conditions are

</details>

---

# Recap

- Rust code should be sound code, and sound code must be memory safe code
- `unsafe` keyword shifts responsibility from the compiler to the programmer
- `unsafe` requires human review

<details>

- soundness implies safety, but safety does not imply soundness
- "safe", "unsafe" and precondition documentation/comments are compile-time
  promises
- "sound" and "unsound" are judgments about whether the code follows the
  agreed-upon rules about marking things "unsafe" and documenting preconditions.
- Unsafe operations and precondition comments are not checked automatically.
