//! ChaCha20Poly1305 is specified in [RFC 7539](https://tools.ietf.org/html/rfc7539).
//!
//! This version of the RFC had [several errata](https://www.rfc-editor.org/errata_search.php?rfc=7539), which led to the new RFC 8439.
//!
//! ## Issue 4371
//! We show that errata #4371 would have been caught by our typechecker.
//! Details of the issue can be found [here](https://www.rfc-editor.org/errata/eid4371).
//!
//! The RFC says
//!
//! ```text
//! 2.8.1.  Pseudocode for the AEAD Construction
//!
//!      pad16(x):
//!         if (len(x) % 16)==0
//!            then return NULL
//!            else return copies(0, 16-(len(x)%16))
//!         end
//!
//!      chacha20_aead_encrypt(aad, key, iv, constant, plaintext):
//!         nonce = constant | iv
//!         otk = poly1305_key_gen(key, nonce)
//!         ciphertext = chacha20_encrypt(key, 1, nonce, plaintext)
//!         mac_data = aad | pad16(aad)
//!         mac_data |= ciphertext | pad16(ciphertext)
//!         mac_data |= num_to_4_le_bytes(aad.length)
//!         mac_data |= num_to_4_le_bytes(ciphertext.length)
//!         tag = poly1305_mac(mac_data, otk)
//!         return (ciphertext, tag)
//! ```
//!
//! `num_to_4_le_bytes` should have been `num_to_8_le_bytes` here though.
//! This is something that hacspec would have caught.
//! In particular, any of the following would not compile.
//! See the [source code](../src/chacha20poly1305_rfc7539/chacha20poly1305.rs.html#61-62)) for the correct implementation.
//!
//! ```ignore
//! padded_msg = padded_msg.update(pad_aad + pad_msg, &U32_to_le_bytes(U64(laad as u64)));
//! padded_msg = padded_msg.update(pad_aad + pad_msg + 8, &U32_to_le_bytes(U64(lmsg as u64)));
//! ```
//!
//! ```ignore
//! padded_msg = padded_msg.update(pad_aad + pad_msg, &U64_to_le_bytes(U32(laad as u64)));
//! padded_msg = padded_msg.update(pad_aad + pad_msg + 8, &U64_to_le_bytes(U32(lmsg as u64)));
//! ```
//!
//! ```ignore
//! padded_msg = padded_msg.update(pad_aad + pad_msg, &U64_to_le_bytes(U64(laad as u32)));
//! padded_msg = padded_msg.update(pad_aad + pad_msg + 8, &U64_to_le_bytes(U64(lmsg as u32)));
//! ```
//!
//! However, writing
//!
//! ```ignore
//! padded_msg = padded_msg.update(pad_aad + pad_msg, &U32_to_le_bytes(U32(laad as u32)));
//! padded_msg = padded_msg.update(pad_aad + pad_msg + 8, &U32_to_le_bytes(U32(lmsg as u32)));
//! ```
//!
//! is valid in Rust and therefore compiles.
//! Even worse, using only a simple test as done here in the demo this does not
//! even fail.
//!

// Import hacspec and all needed definitions.
use hacspec_lib::*;

// Import chacha20 and poly1305
use chacha20::*;
use poly1305::*;

fn pad_aad_msg(aad: &ByteSeq, msg: &ByteSeq) -> ByteSeq {
    let laad = aad.len();
    let lmsg = msg.len();
    let mut pad_aad = 16 * ((laad >> 4u32) + 1);
    let mut pad_msg = 16 * ((lmsg >> 4u32) + 1);
    if laad % 16 == 0 {
        pad_aad = laad;
        pad_msg = lmsg;
    };
    let mut padded_msg = ByteSeq::new(pad_aad + pad_msg + 16);
    padded_msg = padded_msg.update(0, aad);
    padded_msg = padded_msg.update(pad_aad, msg);
    padded_msg = padded_msg.update(pad_aad + pad_msg, &U32_to_le_bytes(U32(laad as u32)));
    padded_msg = padded_msg.update(pad_aad + pad_msg + 8, &U32_to_le_bytes(U32(lmsg as u32)));
    padded_msg
}

pub fn encrypt(key: Key, iv: IV, aad: &ByteSeq, msg: &ByteSeq) -> (ByteSeq, Tag) {
    let key_block = block(key, U32(0u32), iv);
    let mac_key = KeyPoly::from_slice_range(&key_block, 0..32);
    let cipher_text = chacha(key, iv, msg);
    let padded_msg = pad_aad_msg(aad, &cipher_text);
    let tag = poly(&padded_msg, mac_key);
    (cipher_text, tag)
}

pub fn decrypt(
    key: Key,
    iv: IV,
    aad: &ByteSeq,
    cipher_text: &ByteSeq,
    tag: Tag,
) -> (ByteSeq, bool) {
    let key_block = block(key, U32(0u32), iv);
    let mac_key = KeyPoly::from_slice_range(&key_block, 0..32);
    let padded_msg = pad_aad_msg(aad, cipher_text);
    let my_tag = poly(&padded_msg, mac_key);
    let plain_text = chacha(key, iv, cipher_text);
    (plain_text, my_tag == tag)
}
