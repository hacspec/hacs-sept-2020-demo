//! This is a demo of hacspec for the HACS workshop on September 22.
//!
//! ## Overview
//! There are 5 separate parts to this demo.
//!
//! 1) Demo hacspec for some primitive side by side with RFC pseudocode
//!     1) Poly1305
//!     2) Polynomial example
//! 2) Demo the typechecker with some (self-introduced) bug
//! 3) Demo F* translation with a more advanced bug found by F*
//! 4) Demo F* spec side-by-side with existing HACL* spec, and a proof of
//!    equivalence for one function
//!
//! ### Setup
//! The typechecker requires a Rust nightly compiler and additional Rust components.
//!
//! ```bash
//! rustup toolchain install nightly
//! rustup component add --toolchain nightly rustc-dev
//! rustup component add --toolchain nightly llvm-tools-preview
//! ```
//! To run the typechecker and compiler on a spec check out the hacspec repository
//!
//! ```bash
//! git clone https://github.com/hacspec/hacspec.git
//! ```
//!
//! #### HACL* and F*
//! To typecheck the generated F* code [F*](https://www.fstar-lang.org/#download)
//! and a checkout of HACL* is required.
//!
//! ```bash
//! git clone https://github.com/project-everest/hacl-star.git
//! ```
//!
//! **Windows:** note that this hasn't been tested on Windows.
//!
//! ## Demo Specs
//! The demo specs are in separate crates:
//! * [ChaCha20 hacspec](../chacha20/index.html)
//! * [Poly1305 hacspec](../poly1305/index.html)
//! * [ChaCha20Poly1305 hacspec](../chacha20poly1305/index.html)
//! * [Polynomial arithmetic](../poly_demo/index.html)
//!
//! ### Running tests on the spec
//! Each spec has tests in the tests folder.
//! They can be run with `cargo test` in the top level directory or each
//! individual crate.
//!
//! ### Typecheck specs
//! To ensure that a spec is compliant with the hacspec language the [typechecker](https://github.com/hacspec/hacspec/tree/master/language)
//! can be used.
//! The typechecker also outputs F* code.
//!
//! Go into `hacspec/language` run the compiler as follows on the examples in this
//! repository (assuming the `hacspec-hacs-sept-2020-demo` is next to `hacspec`).
//! You can point this to any hacspec implementation.
//!
//! ```bash
//! cargo +nightly run -- \
//!     -L $PWD/../hacspec-hacs-sept-2020-demo/target/debug/deps \
//!     --crate-type=lib --edition=2018 \
//!     --extern=hacspec_lib -Zno-codegen \
//!     $PWD/../../hacspec-hacs-sept-2020-demo/hacspec-examples/poly1305/poly1305.rs
//! ```
//!
//! To generate F* code from hacspec run
//!
//! ```bash
//! cargo +nightly run -- \
//!     -L $PWD/../hacspec-hacs-sept-2020-demo/target/debug/deps \
//!     --crate-type=lib --edition=2018 \
//!     --extern=hacspec_lib \
//!     -o $PWD/../../hacspec-hacs-sept-2020-demo/Hacspec.Poly1305.fst \
//!     $PWD/../../hacspec-hacs-sept-2020-demo/hacspec-examples/poly1305/poly1305.rs
//! ```
//!
//! ### Typecheck F* specs
//! The generated F* specs can be typechecked with F* using [HACL*](https://github.com/project-everest/hacl-star/)
//! and the [hacspec F* library](https://github.com/hacspec/hacspec/tree/master/fstar).
//!
//! Prerequisites:
//! * `fstar.exe` is in your path
//! * `$HACL_HOME` is set to a checkout HACL*
//! * `$HACSPEC_HOME` is set to the checkout of hacspec
//!
//! ```bash
//! fstar.exe --lax \
//!     --include $HACSPEC_HOME/fstar \
//!     --include $HACL_HOME/lib \
//!     Hacspec.Poly1305.fst
//! ```
//!
