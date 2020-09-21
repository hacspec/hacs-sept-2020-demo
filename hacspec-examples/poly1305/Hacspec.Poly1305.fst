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

let le_bytes_to_num
  (b_1858 : byte_seq)
  (start_1859 : uint_size)
  (block_len_1860 : uint_size)
  : uint128 =
  let block_as_u128_1861 =
    seq_from_slice 16 (b_1858) (start_1859) (min (usize 16) (block_len_1860))
  in
  uint128_from_le_bytes (block_as_u128_1861)

let clamp (r_1862 : uint128) : field_element =
  let r_uint_1863 =
    (r_1862) &. (secret (pub_u128 0xffffffc0ffffffc0ffffffc0fffffff))
  in
  nat_from_secret_literal (r_uint_1863)

let encode (block_uint_1864 : uint128) (len_1865 : uint_size) : field_element =
  let w_elem_1866 = nat_from_secret_literal (block_uint_1864) in
  let l_elem_1867 = nat_pow2 ((usize 8) * (len_1865)) in
  (w_elem_1866) + (l_elem_1867)

let num_to_16_le_bytes (a_1868 : field_element) : tag =
  let n_v_1869 = nat_to_public_byte_seq_le (a_1868) in
  let tag_1870 = seq_new_ blocksize in
  let (tag_1870) =
    foldi (usize 0) (
        min (seq_len (tag_1870)) (seq_len #pub_uint8 (n_v_1869))) (fun (
        i_1871,
        (tag_1870)
      ) ->
      let tag_1870 =
        array_upd tag_1870 (i_1871) (array_index (n_v_1869) (i_1871))
      in
      (tag_1870))
    (tag_1870)
  in
  tag_1870

let poly (m_1872 : byte_seq) (key_1873 : key_poly) : tag =
  let key_1874 = seq_from_slice (key_1873) (usize 0) (seq_len (key_1873)) in
  let r_1875 = le_bytes_to_num (key_1874) (usize 0) (blocksize) in
  let r_1876 = clamp (r_1875) in
  let s_1877 = le_bytes_to_num (key_1874) (blocksize) (blocksize) in
  let s_1878 = nat_from_secret_literal (s_1877) in
  let a_1879 = nat_from_literal (pub_u128 0x0) in
  let (a_1879) =
    foldi (usize 0) (seq_num_chunks (m_1872) (blocksize)) (fun (i_1880, (a_1879)
      ) ->
      let (len_1881, block_1882) =
        seq_get_chunk (m_1872) (blocksize) (i_1880)
      in
      let block_uint_1883 = le_bytes_to_num (block_1882) (usize 0) (len_1881) in
      let n_1884 = encode (block_uint_1883) (len_1881) in
      let a_1879 = (a_1879) + (n_1884) in
      let a_1879 = (r_1876) * (a_1879) in
      (a_1879))
    (a_1879)
  in
  let a_1885 = (a_1879) + (s_1878) in
  num_to_16_le_bytes (a_1885)

