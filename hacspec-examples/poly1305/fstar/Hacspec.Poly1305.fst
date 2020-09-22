module Hacspec.Poly1305

#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"

open Hacspec.Lib
open FStar.Mul

type key_poly = lseq (uint8) (usize 32)

let blocksize : uint_size =
  usize 16

type block = lseq (uint8) (blocksize)

type tag = lseq (pub_uint8) (blocksize)

type field_canvas = lseq (pub_uint8) (usize 131)

type field_element = nat_mod 0x03fffffffffffffffffffffffffffffffb

let le_bytes_to_num (b_1858 : byte_seq{seq_len b_1858 <= 16}) : uint128 =
  let len = seq_len b_1858 in
  let b = seq_new_ 16 (u8 0) in
  let b = update_slice b 0 len b_1858 in
  uint128_from_le_bytes b

let le_bytes_16_to_num (b_1858 : byte_seq{seq_len b_1858 == 16}) : uint128 =
  uint128_from_le_bytes b_1858

let clamp (r_1860 : uint128) : field_element =
  let r_uint_1861 =
    (r_1860) &. (secret (pub_u128 0xffffffc0ffffffc0ffffffc0fffffff))
  in
  nat_from_secret_literal (r_uint_1861)

let encode (block_uint_1862 : uint128) (len_1863 : uint_size{len_1863 <= 16}) : field_element =
  let w_elem_1864 = nat_from_secret_literal (block_uint_1862) in
  let l_elem_1865 = nat_pow2 ((usize 8) * (len_1863)) in
  assert_norm (pow2 128 < 0x03fffffffffffffffffffffffffffffffb);
  Math.Lemmas.pow2_le_compat 128 (8 * len_1863);
  (w_elem_1864) +% (l_elem_1865)

let num_to_16_le_bytes (a_1866 : field_element) : tag =
  nat_to_public_byte_seq_le a_1866 (usize 16)

let poly (m_1870 : byte_seq{seq_len m_1870 < max_size_t}) (key_1871 : key_poly) : tag =
  let r_1872 = le_bytes_to_num (slice (key_1871) (usize 0) (blocksize)) in
  let r_1873 = clamp (r_1872) in
  let s_1874 = le_bytes_to_num (slice (key_1871) (blocksize) (2*blocksize)) in
  let s_1875 = nat_from_secret_literal (s_1874) in
  let a_1876 = nat_from_literal (pub_u128 0x0) in
  let (a_1876) =
    foldi (usize 0) (seq_num_chunks (m_1870) (blocksize)) (fun i_1877 (a_1876)
       ->
      let (len_1878, block_1879) =
        seq_get_chunk (m_1870) (blocksize) (i_1877)
      in
      let block_uint_1880 = le_bytes_to_num (block_1879) in
// Uncommenting the following line mimics the bug in Errata 5689
//      let block_uint_1880 = le_bytes_16_to_num (slice m_1870 (blocksize*i_1877) (blocksize*i_1877+blocksize)) in
      let n_1881 = encode (block_uint_1880) (len_1878) in
      let a_1876 = (a_1876) + (n_1881) in
      let a_1876 = (r_1873) * (a_1876) in
      (a_1876))
    (a_1876)
  in
  let a_1882 = (a_1876) + (s_1875) in
  num_to_16_le_bytes (a_1882 % pow2 128)
