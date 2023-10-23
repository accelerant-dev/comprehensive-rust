# Comprehensive Rust 🦀

[![Build workflow](https://img.shields.io/github/actions/workflow/status/google/comprehensive-rust/build.yml?style=flat-square)](https://github.com/google/comprehensive-rust/actions/workflows/build.yml?query=branch%3Amain)
[![GitHub contributors](https://img.shields.io/github/contributors/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/stargazers)

This repository has the source code for the customizations by Accelerant to
Comprehensive Rust 🦀, a multi-day Rust course developed by the Android team.

The course covers all aspects of Rust, from basic syntax to generics and error
handling. It also includes deep dives on [Android], [bare-metal], and
[concurrency].

[Android]: https://google.github.io/comprehensive-rust/android.html
[bare-metal]: https://google.github.io/comprehensive-rust/bare-metal.html
[concurrency]: https://google.github.io/comprehensive-rust/concurrency.html

## Setup

You'll need two core tools:

- Rust
- A text editor

For Rust, you're recommended to use the installer provided at
<https://rustup.rs> which provides `cargo`, `rustup` and `rustc`.

If your editor has support for [dev containers], then you'll be able to benefit
from having a pre-built environment for you. Otherwise you'll need to run the
build steps.

[dev containers]: https://containers.dev/

## Building

The course is built using a few tools:

- [mdbook](https://github.com/rust-lang/mdBook)
- [mdbook-svgbob](https://github.com/boozook/mdbook-svgbob)
- [mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers)
- [mdbook-exerciser](mdbook-exerciser/)
- [mdbook-course](mdbook-course/)

First clone the repository:

```shell
git clone https://github.com/accelerant/comprehensive-rust/
cd comprehensive-rust
git checkout 2023-10
```

Then install these tools with:

```shell
cargo install mdbook
cargo install mdbook-svgbob
cargo install mdbook-i18n-helpers
cargo install --path mdbook-exerciser
cargo install --path mdbook-course
```

Run

```shell
mdbook test
```

to test all included Rust snippets. Run

```shell
mdbook serve
```

to start a web server with the course. You'll find the content on
<http://localhost:3000>. You can use `mdbook build` to create a static version
of the course in the `book/` directory. Note that you have to separately build
and zip exercises and add them to `book/html`. To build any of the translated
versions of the course, run `MDBOOK_BOOK__LANGUAGE=xx mdbook build -d book/xx`
where `xx` is the ISO 639 language code (e.g. `da` for the Danish translation).
[TRANSLATIONS.md](TRANSLATIONS.md) contains further instructions.

> **Note** On Windows, you need to enable symlinks
> (`git config --global core.symlinks true`) and Developer Mode.

## Contact

For questions or comments, please contact [Tim McNamara](mailto:tim@accelerant.dev) in the first instance.

The team at Google is also very supportive. To contribute to the core material,
contact [Martin Geisler](mailto:mgeisler@google.com) or start a
[discussion on GitHub](https://github.com/google/comprehensive-rust/discussions).
We would love to hear from you.
