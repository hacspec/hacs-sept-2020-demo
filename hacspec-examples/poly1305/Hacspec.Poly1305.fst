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
  let tag_1868 = seq_new_ blocksize in
  let (tag_1868) =
    foldi (usize 0) (
        min (seq_len (tag_1868)) (seq_len #pub_uint8 (n_v_1867))) (fun (
        i_1869,
        (tag_1868)
      ) ->
      let tag_1868 =
        array_upd tag_1868 (i_1869) (array_index (n_v_1867) (i_1869))
      in
      (tag_1868))
    (tag_1868)
  in
  tag_1868

let poly (m_1870 : byte_seq) (key_1871 : key_poly) : tag =
  let r_1872 = le_bytes_to_num (seq_slice (key_1871) (usize 0) (blocksize)) in
  let r_1873 = clamp (r_1872) in
  let s_1874 = le_bytes_to_num (seq_slice (key_1871) (blocksize) (blocksize)) in
  let s_1875 = nat_from_secret_literal (s_1874) in
  let a_1876 = nat_from_literal (pub_u128 0x0) in
  let (a_1876) =
    foldi (usize 0) (seq_num_chunks (m_1870) (blocksize)) (fun (i_1877, (a_1876)
      ) ->
      let (len_1878, block_1879) =
        seq_get_chunk (m_1870) (blocksize) (i_1877)
      in
      let block_uint_1880 = le_bytes_to_num (block_1879) in
      let n_1881 = encode (block_uint_1880) (len_1878) in
      let a_1876 = (a_1876) + (n_1881) in
      let a_1876 = (r_1873) * (a_1876) in
      (a_1876))
    (a_1876)
  in
  let a_1882 = (a_1876) + (s_1875) in
  num_to_16_le_bytes (a_1882)

