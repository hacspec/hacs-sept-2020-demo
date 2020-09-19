//! # Poly1305
//!
//! RFC 7539
//! ```text
//! 2.5.1.  The Poly1305 Algorithms in Pseudocode
//!
//!       clamp(r): r &= 0x0ffffffc0ffffffc0ffffffc0fffffff
//!       poly1305_mac(msg, key):
//!          r = (le_bytes_to_num(key[0..15])
//!          clamp(r)
//!          s = le_num(key[16..31])
//!          accumulator = 0
//!          p = (1<<130)-5
//!          for i=1 upto ceil(msg length in bytes / 16)
//!             n = le_bytes_to_num(msg[((i-1)*16)..(i*16)] | [0x01])
//!             a += n
//!             a = (r * a) % p
//!             end
//!          a += s
//!          return num_to_16_le_bytes(a)
//!          end
//! ```
//!
//! RFC 8439
//! ```text
//! 2.5.1.  The Poly1305 Algorithms in Pseudocode
//!
//!       clamp(r): r &= 0x0ffffffc0ffffffc0ffffffc0fffffff
//!       poly1305_mac(msg, key):
//!          r = le_bytes_to_num(key[0..15])
//!          clamp(r)
//!          s = le_bytes_to_num(key[16..31])
//!          a = 0  /* a is the accumulator */
//!          p = (1<<130)-5
//!          for i=1 upto ceil(msg length in bytes / 16)
//!             n = le_bytes_to_num(msg[((i-1)*16)..(i*16)] | [0x01])
//!             a += n
//!             a = (r * a) % p
//!             end
//!          a += s
//!          return num_to_16_le_bytes(a)
//!          end
//! ```
//!
//! ## Errata ID: 5689
//! Errata [5689](https://www.rfc-editor.org/errata/eid5689) identifies an issue
//! where the input msg end is overrun if it's not a multiple of 16.
//!
//! RFC 8439 takes `msg[((i-1)*16)..(i*16)]` when the end should be at
//! `min(i*16-1, msg length in bytes - 1)`, not `i*16`.
//!
//! Using `num_chunks` and `get_chunk` provided by the hacspec library such an
//! error is not possible.
//! `get_chunk` returns a `Seq` of length up to the provided chunk size as well
//! as the actual length of the chunk.
//! This makes such an error less likely but still possible.
//!

// Import hacspec and all needed definitions.
use hacspec_lib::*;

bytes!(KeyPoly, 32);

const BLOCKSIZE: usize = 16;
// Type definitions for use in poly1305.

// These are type aliases for convenience
bytes!(Block, BLOCKSIZE);

// These are actual types; fixed-length arrays.
public_bytes!(Tag, BLOCKSIZE);

// This defines the field for modulo 2^130-5.
// In particular `FieldElement` and `FieldCanvas` are defined.
// The `FieldCanvas` is an integer type with 131-bit (to hold 2*(2^130-5)).
// The `FieldElement` is a natural integer modulo 2^130-5.
//
// XXX: The types are public here but should be secret. But secret BigNums are
// not implemented yet.
public_nat_mod!(
    FieldElement,
    FieldCanvas,
    131,
    "03fffffffffffffffffffffffffffffffb"
);

/// Take a `KeyPoly` (32-byte array) and convert it into two U128 (secret u128).
pub fn key_to_uints(b: KeyPoly) -> (U128, U128) {
    let uint_128_1 = U128Word::from_slice(&b, 0, BLOCKSIZE);
    let uint_128_2 = U128Word::from_slice(&b, BLOCKSIZE, BLOCKSIZE);
    (
        U128_from_le_bytes(uint_128_1),
        U128_from_le_bytes(uint_128_2),
    )
}

/// Take a variable length byte array and convert it into a U128 (secret u128).
pub fn seq_to_uint(b: &ByteSeq, block_len: usize) -> U128 {
    let block_as_u128 = U128Word::from_slice(b, 0, min(16, block_len));
    U128_from_le_bytes(block_as_u128)
}

/// Clamp a block `r` and return the resulting `FieldElement`.
pub fn clamp(r: U128) -> FieldElement {
    let r_uint = r & U128(0x0fff_fffc_0fff_fffc_0fff_fffc_0fff_ffffu128);
    FieldElement::from_secret_literal(r_uint)
}

/// Convert a block (part of the byte sequence) to a `FieldElement`.
pub fn le_bytes_to_num(block: &ByteSeq, len: usize) -> FieldElement {
    let block_uint = seq_to_uint(block, len);
    let w_elem = FieldElement::from_secret_literal(block_uint);
    let l_elem = FieldElement::pow2(8 * len);
    w_elem + l_elem
}

/// Convert a `FieldElement` to a `Tag` (16-byte array).
pub fn num_to_16_le_bytes(a: FieldElement) -> Tag {
    // Note that a might be less than 16 byte -> zero-pad; but might also be
    // larger than Tag::capacity().
    let n_v = a.to_public_byte_seq_le();
    let mut tag = Tag::new();
    for i in 0..min(tag.len(), n_v.len()) {
        tag[i] = n_v[i];
    }
    tag
}

pub fn poly(m: &ByteSeq, key: KeyPoly) -> Tag {
    let (key_1, key_2) = key_to_uints(key);

    let r_elem = clamp(key_1);
    let s_elem = FieldElement::from_secret_literal(key_2);

    let mut a = FieldElement::from_literal(0u128);
    for i in 0..m.num_chunks(BLOCKSIZE) {
        let (len, block) = m.get_chunk(BLOCKSIZE, i);
        let n = le_bytes_to_num(&block, len);
        a = a + n;
        a = a * r_elem;
    }

    let a = a + s_elem;
    num_to_16_le_bytes(a)
}
