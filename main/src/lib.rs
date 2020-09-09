//! This is a demo of hacspec for the HACS workshop on September 22.
//! 
//! ## Overview
//! There are 5 separate parts to this demo.
//! 
//! 1) Demo hacspec for some primitive side by side with RFC pseudocode
//! 2) Demo the typechecker with some (self-introduced) bug
//! 3) Demo F* translation with a more advanced bug found by F*
//! 4) Demo F* spec side-by-side with existing HACL* spec, and a proof of
//!    equivalence for one function
//! 5) Present formal semantics for hacspec that enables us to do all of the
//!    above in a sound way.
//! 
//! ## Demo Specs
//! The demo specs are in separate crates:
//! * [ChaCha20 hacspec](../hacspec_chacha20/index.html)
//! * [Poly1305 hacspec](../hacspec_poly1305/index.html)
