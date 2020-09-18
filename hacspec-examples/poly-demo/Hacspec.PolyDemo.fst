module Hacspec.PolyDemo

#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"

open Hacspec.Lib
open FStar.Mul

let build_irreducible (p_1842 : uint_size) : seq pub_int128 =
  let irr_1843 = seq_new_ #pub_int128 ((p_1842) + (usize 1)) in
  let irr_1843 = array_upd irr_1843 (usize 0) (- (pub_i128 0x1)) in
  let irr_1843 = array_upd irr_1843 (usize 1) (- (pub_i128 0x1)) in
  let irr_1843 = array_upd irr_1843 (p_1842) (pub_i128 0x1) in
  irr_1843

let round_to_3
  (poly_1844 : seq pub_int128)
  (q_1845 : pub_int128)
  : seq pub_int128 =
  let result_1846 = seq_clone #pub_int128 (poly_1844) in
  let q_12_1847 = ((q_1845) -. (pub_i128 0x1)) /. (pub_i128 0x2) in
  let (result_1846) =
    foldi (usize 0) (seq_len #pub_int128 (poly_1844)) (fun (
        i_1848,
        (result_1846)
      ) ->
      let (result_1846) =
        if (array_index (poly_1844) (i_1848)) >. (q_12_1847) then begin
          let result_1846 =
            array_upd result_1846 (i_1848) (
              (array_index (poly_1844) (i_1848)) -. (q_1845))
          in
          (result_1846)
        end else begin (result_1846)
        end
      in
      (result_1846))
    (result_1846)
  in
  let (result_1846) =
    foldi (usize 0) (seq_len #pub_int128 (result_1846)) (fun (
        i_1849,
        (result_1846)
      ) ->
      let (result_1846) =
        if ((array_index (result_1846) (i_1849)) %. (pub_i128 0x3)) != (
          pub_i128 0x0) then begin
          let result_1846 =
            array_upd result_1846 (i_1849) (
              (array_index (result_1846) (i_1849)) -. (pub_i128 0x1))
          in
          let (result_1846) =
            if ((array_index (result_1846) (i_1849)) %. (pub_i128 0x3)) != (
              pub_i128 0x0) then begin
              let result_1846 =
                array_upd result_1846 (i_1849) (
                  (array_index (result_1846) (i_1849)) +. (pub_i128 0x2))
              in
              (result_1846)
            end else begin (result_1846)
            end
          in
          (result_1846)
        end else begin (result_1846)
        end
      in
      (result_1846))
    (result_1846)
  in
  result_1846

let encrypt
  (r_1850 : seq pub_int128)
  (h_1851 : seq pub_int128)
  (q_1852 : pub_int128)
  (irreducible_1853 : seq pub_int128)
  : seq pub_int128 =
  let pre_1854 = mul_poly_irr (r_1850) (h_1851) (irreducible_1853) (q_1852) in
  round_to_3 (pre_1854) (q_1852)

let poly_encrypt
  (r_1855 : seq pub_int128)
  (h_1856 : seq pub_int128)
  : seq pub_int128 =
  let p_1857 = usize 653 in
  let q_1858 = pub_i128 0x120d in
  let w_1859 = usize 288 in
  let irreducible_1860 = build_irreducible (p_1857) in
  encrypt (r_1855) (h_1856) (q_1858) (irreducible_1860)

let triple_poly
  (poly_1861 : seq pub_int128)
  (modulus_1862 : pub_int128)
  : seq pub_int128 =
  add_poly (poly_1861) (add_poly (poly_1861) (poly_1861) (modulus_1862)) (
    modulus_1862)

let poly_decrypt
  (c_1863 : seq pub_int128)
  (key_f_1864 : seq pub_int128)
  (key_v_1865 : seq pub_int128)
  : (seq pub_int128 & bool) =
  let p_1866 = usize 653 in
  let q_1867 = pub_i128 0x120d in
  let w_1868 = usize 288 in
  let irreducible_1869 = build_irreducible (p_1866) in
  let f_c_1870 =
    mul_poly_irr (key_f_1864) (c_1863) (irreducible_1869) (q_1867)
  in
  let (f_3_c_1871, ok_decrypt_1872) =
    poly_to_ring (irreducible_1869) (triple_poly (f_c_1870) (q_1867)) (q_1867)
  in
  let q_12_1873 = ((q_1867) -. (pub_i128 0x1)) /. (pub_i128 0x2) in
  let (f_3_c_1871) =
    foldi (usize 0) (seq_len #pub_int128 (f_3_c_1871)) (fun (
        i_1874,
        (f_3_c_1871)
      ) ->
      let (f_3_c_1871) =
        if (array_index (f_3_c_1871) (i_1874)) >. (q_12_1873) then begin
          let f_3_c_1871 =
            array_upd f_3_c_1871 (i_1874) (
              (array_index (f_3_c_1871) (i_1874)) -. (q_1867))
          in
          (f_3_c_1871)
        end else begin (f_3_c_1871)
        end
      in
      (f_3_c_1871))
    (f_3_c_1871)
  in
  let e_1875 = seq_new_ #pub_int128 (seq_len #pub_int128 (f_3_c_1871)) in
  let (e_1875) =
    foldi (usize 0) (seq_len #pub_int128 (e_1875)) (fun (i_1876, (e_1875)) ->
      let e_1875 =
        array_upd e_1875 (i_1876) (
          (array_index (f_3_c_1871) (i_1876)) %. (pub_i128 0x3))
      in
      (e_1875))
    (e_1875)
  in
  let e_1875 = make_positive (e_1875) (pub_i128 0x3) in
  let r_1877 =
    mul_poly_irr (e_1875) (key_v_1865) (irreducible_1869) (pub_i128 0x3)
  in
  let (r_1877) =
    foldi (usize 0) (seq_len #pub_int128 (r_1877)) (fun (i_1878, (r_1877)) ->
      let (r_1877) =
        if (array_index (r_1877) (i_1878)) == (pub_i128 0x2) then begin
          let r_1877 = array_upd r_1877 (i_1878) (- (pub_i128 0x1)) in
          (r_1877)
        end else begin (r_1877)
        end
      in
      (r_1877))
    (r_1877)
  in
  (r_1877, ok_decrypt_1872)

