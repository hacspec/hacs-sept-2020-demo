//! This is a demo of hacspec for the HACS workshop on September 22.
//!
//! ## Agenda
//! 1) hacspec overview (FK)
//! 2) "rustspec" (DM)
//!
//! ## Takeaways
//! * hacspec is a Rust subset to write succinct specifications of crypto primitives
//!   * eventually also protocols
//! * [formalization and semantics](https://github.com/hacspec/hacspec/blob/master/formalization/formalization.pdf)
//! * usable for PQC
//!   * still WiP and there are rough edges
//!   * write hacspec for PQC things and tell us what's not working
//! * usable right now
//!   * but typechecker still work in progress
//!
//! ### Workflow
//! 1. Write
//! * write spec (look at [examples](https://github.com/hacspec/hacspec/tree/master/examples) or [formalization](https://github.com/hacspec/hacspec/blob/master/formalization/formalization.pdf) for allowed subset)
//! * write and run tests
//! * run [typechecker](#typecheck-specs)
//! 2. Use & implement
//! * use spec in any Rust (or FFI) application
//! * implement optimised version with same API
//! * swap out implementation
//! 3. Proofs
//! * proof equivalence of spec and implementation
//!
//! ## Setup hacspec
//! First check out the hacspec repository.
//! It contains the typechecker, compiler, hacspec library, and examples.
//!
//! ```bash
//! git clone https://github.com/hacspec/hacspec.git
//! ```
//! The typechecker requires a Rust nightly compiler and additional Rust components.
//!
//! ```bash
//! rustup toolchain install nightly
//! rustup component add --toolchain nightly rustc-dev
//! rustup component add --toolchain nightly llvm-tools-preview
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
//! **Note:** Hacspec has not been tested on Windows.
//!
//! ## Demo Overview
//! There are 4 separate parts to this demo.
//!
//! 1) Demo hacspec for some primitive compared to RFC pseudocode
//!     1) Poly1305
//!     2) Polynomial example
//! 2) Demo of the typechecker
//! 3) Demo F* translation
//! 4) Demo F* spec side-by-side with existing HACL* spec, and a proof of
//!    equivalence for one function
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
//! #### Multi-crate specs
//!
//! ```bash
//! cargo +nightly run -- \
//!     -L $PWD/../hacspec-hacs-sept-2020-demo/target/debug/deps \
//!     --crate-type=lib --edition=2018 \
//!     --extern=hacspec_lib \
//!     --extern=chacha20 \
//!     --extern=poly1305 \
//!     -Zno-codegen \
//!     $PWD/../../hacspec-hacs-sept-2020-demo/hacspec-examples/chacha20poly1305-rfc7539/src/chacha20poly1305.rs
//! ```
//!
//! Note that ChaCha20Poly1305 can't be translated to F* just yet.
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
