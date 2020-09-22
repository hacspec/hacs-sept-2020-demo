module Hacspec.Poly1305-bug

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

let le_bytes_to_num (b_1858 : byte_seq) : uint128 =
  let block_as_u128_1859 =
    seq_from_slice 16 (b_1858) (usize 0) (min (blocksize) (seq_len (b_1858)))
  in
  uint128_from_le_bytes (block_as_u128_1859)

let clamp (r_1860 : uint128) : field_element =
  let r_uint_1861 =
    (r_1860) &. (secret (pub_u128 0xffffffc0ffffffc0ffffffc0fffffff))
  in
  nat_from_secret_literal (r_uint_1861)

let encode (block_uint_1862 : uint128) (len_1863 : uint_size) : field_element =
  let w_elem_1864 = nat_from_secret_literal (block_uint_1862) in
  let l_elem_1865 = nat_pow2 ((usize 8) * (len_1863)) in
  (w_elem_1864) + (l_elem_1865)

let num_to_16_le_bytes (a_1866 : field_element) : tag =
  let n_v_1867 = nat_to_public_byte_seq_le (a_1866) in
  seq_from_seq (seq_slice #pub_uint8 (n_v_1867) (usize 0) (blocksize))

let poly (m_1868 : byte_seq) (key_1869 : key_poly) : tag =
  let r_1870 = le_bytes_to_num (seq_slice (key_1869) (usize 0) (blocksize)) in
  let r_1871 = clamp (r_1870) in
  let s_1872 = le_bytes_to_num (seq_slice (key_1869) (blocksize) (blocksize)) in
  let s_1873 = nat_from_secret_literal (s_1872) in
  let a_1874 = nat_from_literal (pub_u128 0x0) in
  let (a_1874) =
    foldi (usize 0) (seq_num_chunks (m_1868) (blocksize)) (fun (i_1875, (a_1874)
      ) ->
      let block_uint_1876 =
        le_bytes_to_num (
          seq_slice (m_1868) ((blocksize) * (i_1875)) (blocksize))
      in
      let n_1877 = encode (block_uint_1876) (blocksize) in
      let a_1874 = (a_1874) + (n_1877) in
      let a_1874 = (r_1871) * (a_1874) in
      (a_1874))
    (a_1874)
  in
  let a_1878 = (a_1874) + (s_1873) in
  num_to_16_le_bytes (a_1878)

