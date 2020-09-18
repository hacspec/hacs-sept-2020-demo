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

let key_to_uints (b_1864 : key_poly) : (uint128 & uint128) =
  let uint_128_1_1865 = seq_from_slice 16 (b_1864) (usize 0) (blocksize) in
  let uint_128_2_1866 = seq_from_slice 16 (b_1864) (blocksize) (blocksize) in
  (
    uint128_from_le_bytes (uint_128_1_1865),
    uint128_from_le_bytes (uint_128_2_1866)
  )

let seq_to_uint (b_1867 : byte_seq) : uint128 =
  let block_len_1868 = seq_len (b_1867) in
  let block_as_u128_1869 =
    seq_from_slice 16 (b_1867) (usize 0) (min (usize 16) (block_len_1868))
  in
  uint128_from_le_bytes (block_as_u128_1869)

let clamp (r_1870 : uint128) : field_element =
  let r_uint_1871 =
    (r_1870) &. (secret (pub_u128 0xffffffc0ffffffc0ffffffc0fffffff))
  in
  nat_from_secret_literal (r_uint_1871)

let le_bytes_to_num (block_1872 : byte_seq) : field_element =
  let block_uint_1873 = seq_to_uint (block_1872) in
  let w_elem_1874 = nat_from_secret_literal (block_uint_1873) in
  let l_elem_1875 = nat_pow2 ((usize 8) * (seq_len (block_1872))) in
  (w_elem_1874) + (l_elem_1875)

let num_to_16_le_bytes (a_1876 : field_element) : tag =
  let n_v_1877 = nat_to_public_byte_seq_le (a_1876) in
  let tag_1878 = seq_new_ blocksize in
  let (tag_1878) =
    foldi (usize 0) (
        min (seq_len (tag_1878)) (seq_len #pub_uint8 (n_v_1877))) (fun (
        i_1879,
        (tag_1878)
      ) ->
      let tag_1878 =
        array_upd tag_1878 (i_1879) (array_index (n_v_1877) (i_1879))
      in
      (tag_1878))
    (tag_1878)
  in
  tag_1878

let poly (m_1880 : byte_seq) (key_1881 : key_poly) : tag =
  let (key_1_1882, key_2_1883) = key_to_uints (key_1881) in
  let r_elem_1884 = clamp (key_1_1882) in
  let s_elem_1885 = nat_from_secret_literal (key_2_1883) in
  let a_1886 = nat_from_literal (pub_u128 0x0) in
  let (a_1886) =
    foldi (usize 0) (seq_num_chunks (m_1880) (blocksize)) (fun (i_1887, (a_1886)
      ) ->
      let (_, block_1888) = seq_get_chunk (m_1880) (blocksize) (i_1887) in
      let n_1889 = le_bytes_to_num (block_1888) in
      let a_1886 = (a_1886) + (n_1889) in
      let a_1886 = (a_1886) * (r_elem_1884) in
      (a_1886))
    (a_1886)
  in
  let a_1890 = (a_1886) + (s_elem_1885) in
  num_to_16_le_bytes (a_1890)

