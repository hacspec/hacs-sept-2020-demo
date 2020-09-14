var searchIndex = JSON.parse('{\
"chacha20":{"doc":"ChaCha20 is specified in RFC 8439.","i":[[3,"State","chacha20","Fixed length byte array.",null,null],[12,"0","","",0,null],[3,"StateBytes","","Fixed length byte array.",null,null],[12,"0","","",1,null],[3,"IV","","Fixed length byte array.",null,null],[12,"0","","",2,null],[3,"Key","","Fixed length byte array.",null,null],[12,"0","","",3,null],[5,"state_to_bytes","","",null,[[["state",3]],["statebytes",3]]],[5,"quarter_round","","",null,[[["state",3]],["state",3]]],[5,"block_init","","",null,[[["key",3],["u32",3],["iv",3]],["state",3]]],[5,"block_inner","","",null,[[["key",3],["u32",3],["iv",3]],["state",3]]],[5,"block","","",null,[[["key",3],["u32",3],["iv",3]],["statebytes",3]]],[5,"chacha","","",null,[[["key",3],["byteseq",6],["iv",3]],["byteseq",6]]],[11,"new","","",0,[[]]],[11,"length","","",0,[[]]],[11,"from_array","","",0,[[]]],[11,"from_native_slice","","",0,[[]]],[11,"capacity","","",0,[[]]],[11,"from_slice","","",0,[[]]],[11,"from_slice_range","","",0,[[["range",3]]]],[11,"slice","","",0,[[],[["u32",3],["seq",3]]]],[11,"slice_range","","",0,[[["range",3]],[["u32",3],["seq",3]]]],[11,"num_chunks","","",0,[[]]],[11,"get_chunk_len","","",0,[[]]],[11,"get_chunk","","",0,[[]]],[11,"set_chunk","","",0,[[]]],[11,"from_vec","","",0,[[["vec",3],["u32",3]],["state",3]]],[11,"from_seq","","",0,[[],["state",3]]],[11,"from_hex","","Read hex string to Bytes.",0,[[],["state",3]]],[11,"declassify_eq","","",0,[[]]],[11,"to_be_bytes","","",0,[[],[["u8",3],["seq",3]]]],[11,"to_le_bytes","","",0,[[],[["u8",3],["seq",3]]]],[11,"from_public_slice","","",0,[[],["state",3]]],[11,"from_public_array","","Create an array from a regular Rust array.",0,[[],["state",3]]],[11,"new","","",1,[[]]],[11,"length","","",1,[[]]],[11,"from_array","","",1,[[]]],[11,"from_native_slice","","",1,[[]]],[11,"capacity","","",1,[[]]],[11,"from_slice","","",1,[[]]],[11,"from_slice_range","","",1,[[["range",3]]]],[11,"slice","","",1,[[],[["u8",3],["seq",3]]]],[11,"slice_range","","",1,[[["range",3]],[["u8",3],["seq",3]]]],[11,"num_chunks","","",1,[[]]],[11,"get_chunk_len","","",1,[[]]],[11,"get_chunk","","",1,[[]]],[11,"set_chunk","","",1,[[]]],[11,"from_vec","","",1,[[["vec",3],["u8",3]],["statebytes",3]]],[11,"from_seq","","",1,[[],["statebytes",3]]],[11,"from_hex","","Read hex string to Bytes.",1,[[],["statebytes",3]]],[11,"declassify_eq","","",1,[[]]],[11,"to_be_bytes","","",1,[[],[["u8",3],["seq",3]]]],[11,"to_le_bytes","","",1,[[],[["u8",3],["seq",3]]]],[11,"from_public_slice","","",1,[[],["statebytes",3]]],[11,"from_public_array","","Create an array from a regular Rust array.",1,[[],["statebytes",3]]],[11,"to_be_U32s","","",1,[[],[["u32",3],["seq",3]]]],[11,"to_le_U32s","","",1,[[],[["u32",3],["seq",3]]]],[11,"to_be_U64s","","",1,[[],[["seq",3],["u64",3]]]],[11,"to_le_U64s","","",1,[[],[["seq",3],["u64",3]]]],[11,"to_U128s_be","","",1,[[],[["u128",3],["seq",3]]]],[11,"to_U128s_le","","",1,[[],[["u128",3],["seq",3]]]],[11,"to_hex","","",1,[[],["string",3]]],[11,"new","","",2,[[]]],[11,"length","","",2,[[]]],[11,"from_array","","",2,[[]]],[11,"from_native_slice","","",2,[[]]],[11,"capacity","","",2,[[]]],[11,"from_slice","","",2,[[]]],[11,"from_slice_range","","",2,[[["range",3]]]],[11,"slice","","",2,[[],[["u8",3],["seq",3]]]],[11,"slice_range","","",2,[[["range",3]],[["u8",3],["seq",3]]]],[11,"num_chunks","","",2,[[]]],[11,"get_chunk_len","","",2,[[]]],[11,"get_chunk","","",2,[[]]],[11,"set_chunk","","",2,[[]]],[11,"from_vec","","",2,[[["vec",3],["u8",3]],["iv",3]]],[11,"from_seq","","",2,[[],["iv",3]]],[11,"from_hex","","Read hex string to Bytes.",2,[[],["iv",3]]],[11,"declassify_eq","","",2,[[]]],[11,"to_be_bytes","","",2,[[],[["u8",3],["seq",3]]]],[11,"to_le_bytes","","",2,[[],[["u8",3],["seq",3]]]],[11,"from_public_slice","","",2,[[],["iv",3]]],[11,"from_public_array","","Create an array from a regular Rust array.",2,[[],["iv",3]]],[11,"to_be_U32s","","",2,[[],[["u32",3],["seq",3]]]],[11,"to_le_U32s","","",2,[[],[["u32",3],["seq",3]]]],[11,"to_be_U64s","","",2,[[],[["seq",3],["u64",3]]]],[11,"to_le_U64s","","",2,[[],[["seq",3],["u64",3]]]],[11,"to_U128s_be","","",2,[[],[["u128",3],["seq",3]]]],[11,"to_U128s_le","","",2,[[],[["u128",3],["seq",3]]]],[11,"to_hex","","",2,[[],["string",3]]],[11,"new","","",3,[[]]],[11,"length","","",3,[[]]],[11,"from_array","","",3,[[]]],[11,"from_native_slice","","",3,[[]]],[11,"capacity","","",3,[[]]],[11,"from_slice","","",3,[[]]],[11,"from_slice_range","","",3,[[["range",3]]]],[11,"slice","","",3,[[],[["u8",3],["seq",3]]]],[11,"slice_range","","",3,[[["range",3]],[["u8",3],["seq",3]]]],[11,"num_chunks","","",3,[[]]],[11,"get_chunk_len","","",3,[[]]],[11,"get_chunk","","",3,[[]]],[11,"set_chunk","","",3,[[]]],[11,"from_vec","","",3,[[["vec",3],["u8",3]],["key",3]]],[11,"from_seq","","",3,[[],["key",3]]],[11,"from_hex","","Read hex string to Bytes.",3,[[],["key",3]]],[11,"declassify_eq","","",3,[[]]],[11,"to_be_bytes","","",3,[[],[["u8",3],["seq",3]]]],[11,"to_le_bytes","","",3,[[],[["u8",3],["seq",3]]]],[11,"from_public_slice","","",3,[[],["key",3]]],[11,"from_public_array","","Create an array from a regular Rust array.",3,[[],["key",3]]],[11,"to_be_U32s","","",3,[[],[["u32",3],["seq",3]]]],[11,"to_le_U32s","","",3,[[],[["u32",3],["seq",3]]]],[11,"to_be_U64s","","",3,[[],[["seq",3],["u64",3]]]],[11,"to_le_U64s","","",3,[[],[["seq",3],["u64",3]]]],[11,"to_U128s_be","","",3,[[],[["u128",3],["seq",3]]]],[11,"to_U128s_le","","",3,[[],[["u128",3],["seq",3]]]],[11,"to_hex","","",3,[[],["string",3]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"clone","","",0,[[],["state",3]]],[11,"clone","","",1,[[],["statebytes",3]]],[11,"clone","","",2,[[],["iv",3]]],[11,"clone","","",3,[[],["key",3]]],[11,"default","","",0,[[]]],[11,"default","","",1,[[]]],[11,"default","","",2,[[]]],[11,"default","","",3,[[]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"rem","","",0,[[["state",3]],["state",3]]],[11,"rem","","",1,[[["statebytes",3]],["statebytes",3]]],[11,"rem","","",2,[[["iv",3]],["iv",3]]],[11,"rem","","",3,[[["key",3]],["key",3]]],[11,"sub","","",0,[[["state",3]],["state",3]]],[11,"sub","","",1,[[["statebytes",3]],["statebytes",3]]],[11,"sub","","",2,[[["iv",3]],["iv",3]]],[11,"sub","","",3,[[["key",3]],["key",3]]],[11,"add","","",0,[[["state",3]],["state",3]]],[11,"add","","",1,[[["statebytes",3]],["statebytes",3]]],[11,"add","","",2,[[["iv",3]],["iv",3]]],[11,"add","","",3,[[["key",3]],["key",3]]],[11,"mul","","",0,[[["state",3]],["state",3]]],[11,"mul","","",1,[[["statebytes",3]],["statebytes",3]]],[11,"mul","","",2,[[["iv",3]],["iv",3]]],[11,"mul","","",3,[[["key",3]],["key",3]]],[11,"not","","",0,[[]]],[11,"not","","",1,[[]]],[11,"not","","",2,[[]]],[11,"not","","",3,[[]]],[11,"bitand","","",0,[[]]],[11,"bitand","","",1,[[]]],[11,"bitand","","",2,[[]]],[11,"bitand","","",3,[[]]],[11,"bitor","","",0,[[]]],[11,"bitor","","",1,[[]]],[11,"bitor","","",2,[[]]],[11,"bitor","","",3,[[]]],[11,"bitxor","","",0,[[]]],[11,"bitxor","","",1,[[]]],[11,"bitxor","","",2,[[]]],[11,"bitxor","","",3,[[]]],[11,"shl","","",0,[[]]],[11,"shl","","",1,[[]]],[11,"shl","","",2,[[]]],[11,"shl","","",3,[[]]],[11,"shr","","",0,[[]]],[11,"shr","","",1,[[]]],[11,"shr","","",2,[[]]],[11,"shr","","",3,[[]]],[11,"index","","",0,[[],["u32",3]]],[11,"index","","",0,[[],["u32",3]]],[11,"index","","",0,[[],["u32",3]]],[11,"index","","",0,[[],["u32",3]]],[11,"index","","",0,[[["rangefull",3]]]],[11,"index","","",1,[[],["u8",3]]],[11,"index","","",1,[[],["u8",3]]],[11,"index","","",1,[[],["u8",3]]],[11,"index","","",1,[[],["u8",3]]],[11,"index","","",1,[[["rangefull",3]]]],[11,"index","","",2,[[],["u8",3]]],[11,"index","","",2,[[],["u8",3]]],[11,"index","","",2,[[],["u8",3]]],[11,"index","","",2,[[],["u8",3]]],[11,"index","","",2,[[["rangefull",3]]]],[11,"index","","",3,[[],["u8",3]]],[11,"index","","",3,[[],["u8",3]]],[11,"index","","",3,[[],["u8",3]]],[11,"index","","",3,[[],["u8",3]]],[11,"index","","",3,[[["rangefull",3]]]],[11,"index_mut","","",0,[[],["u32",3]]],[11,"index_mut","","",0,[[],["u32",3]]],[11,"index_mut","","",0,[[],["u32",3]]],[11,"index_mut","","",0,[[],["u32",3]]],[11,"index_mut","","",1,[[],["u8",3]]],[11,"index_mut","","",1,[[],["u8",3]]],[11,"index_mut","","",1,[[],["u8",3]]],[11,"index_mut","","",1,[[],["u8",3]]],[11,"index_mut","","",2,[[],["u8",3]]],[11,"index_mut","","",2,[[],["u8",3]]],[11,"index_mut","","",2,[[],["u8",3]]],[11,"index_mut","","",2,[[],["u8",3]]],[11,"index_mut","","",3,[[],["u8",3]]],[11,"index_mut","","",3,[[],["u8",3]]],[11,"index_mut","","",3,[[],["u8",3]]],[11,"index_mut","","",3,[[],["u8",3]]],[11,"create","","",0,[[]]],[11,"len","","",0,[[]]],[11,"iter","","",0,[[],[["iter",3],["u32",3]]]],[11,"update_slice","","",0,[[]]],[11,"update","","",0,[[]]],[11,"update_start","","",0,[[]]],[11,"create","","",1,[[]]],[11,"len","","",1,[[]]],[11,"iter","","",1,[[],[["u8",3],["iter",3]]]],[11,"update_slice","","",1,[[]]],[11,"update","","",1,[[]]],[11,"update_start","","",1,[[]]],[11,"create","","",2,[[]]],[11,"len","","",2,[[]]],[11,"iter","","",2,[[],[["u8",3],["iter",3]]]],[11,"update_slice","","",2,[[]]],[11,"update","","",2,[[]]],[11,"update_start","","",2,[[]]],[11,"create","","",3,[[]]],[11,"len","","",3,[[]]],[11,"iter","","",3,[[],[["u8",3],["iter",3]]]],[11,"update_slice","","",3,[[]]],[11,"update","","",3,[[]]],[11,"update_start","","",3,[[]]],[11,"sub_mod","","(self - rhs) % n.",0,[[]]],[11,"add_mod","","`(self + rhs) % n`",0,[[]]],[11,"mul_mod","","`(self * rhs) % n`",0,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",0,[[]]],[11,"modulo","","`self % n`",0,[[]]],[11,"signed_modulo","","",0,[[]]],[11,"absolute","","`|self|` (coefficient-wise)",0,[[]]],[11,"sub_mod","","(self - rhs) % n.",1,[[]]],[11,"add_mod","","`(self + rhs) % n`",1,[[]]],[11,"mul_mod","","`(self * rhs) % n`",1,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",1,[[]]],[11,"modulo","","`self % n`",1,[[]]],[11,"signed_modulo","","",1,[[]]],[11,"absolute","","`|self|` (coefficient-wise)",1,[[]]],[11,"sub_mod","","(self - rhs) % n.",2,[[]]],[11,"add_mod","","`(self + rhs) % n`",2,[[]]],[11,"mul_mod","","`(self * rhs) % n`",2,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",2,[[]]],[11,"modulo","","`self % n`",2,[[]]],[11,"signed_modulo","","",2,[[]]],[11,"absolute","","`|self|` (coefficient-wise)",2,[[]]],[11,"sub_mod","","(self - rhs) % n.",3,[[]]],[11,"add_mod","","`(self + rhs) % n`",3,[[]]],[11,"mul_mod","","`(self * rhs) % n`",3,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",3,[[]]],[11,"modulo","","`self % n`",3,[[]]],[11,"signed_modulo","","",3,[[]]],[11,"absolute","","`|self|` (coefficient-wise)",3,[[]]],[11,"max_val","","Return largest value that can be represented.",0,[[]]],[11,"wrap_add","","",0,[[]]],[11,"wrap_sub","","",0,[[]]],[11,"wrap_mul","","",0,[[]]],[11,"wrap_div","","",0,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",0,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",0,[[]]],[11,"divide","","Division.",0,[[]]],[11,"inv","","Invert self modulo n.",0,[[]]],[11,"equal","","",0,[[]]],[11,"greater_than","","",0,[[]]],[11,"greater_than_or_qual","","",0,[[]]],[11,"less_than","","",0,[[]]],[11,"less_than_or_equal","","",0,[[]]],[11,"not_equal_bm","","",0,[[]]],[11,"equal_bm","","",0,[[]]],[11,"greater_than_bm","","",0,[[]]],[11,"greater_than_or_equal_bm","","",0,[[]]],[11,"less_than_bm","","",0,[[]]],[11,"less_than_or_equal_bm","","",0,[[]]],[11,"max_val","","Return largest value that can be represented.",1,[[]]],[11,"wrap_add","","",1,[[]]],[11,"wrap_sub","","",1,[[]]],[11,"wrap_mul","","",1,[[]]],[11,"wrap_div","","",1,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",1,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",1,[[]]],[11,"divide","","Division.",1,[[]]],[11,"inv","","Invert self modulo n.",1,[[]]],[11,"equal","","",1,[[]]],[11,"greater_than","","",1,[[]]],[11,"greater_than_or_qual","","",1,[[]]],[11,"less_than","","",1,[[]]],[11,"less_than_or_equal","","",1,[[]]],[11,"not_equal_bm","","",1,[[]]],[11,"equal_bm","","",1,[[]]],[11,"greater_than_bm","","",1,[[]]],[11,"greater_than_or_equal_bm","","",1,[[]]],[11,"less_than_bm","","",1,[[]]],[11,"less_than_or_equal_bm","","",1,[[]]],[11,"max_val","","Return largest value that can be represented.",2,[[]]],[11,"wrap_add","","",2,[[]]],[11,"wrap_sub","","",2,[[]]],[11,"wrap_mul","","",2,[[]]],[11,"wrap_div","","",2,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",2,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",2,[[]]],[11,"divide","","Division.",2,[[]]],[11,"inv","","Invert self modulo n.",2,[[]]],[11,"equal","","",2,[[]]],[11,"greater_than","","",2,[[]]],[11,"greater_than_or_qual","","",2,[[]]],[11,"less_than","","",2,[[]]],[11,"less_than_or_equal","","",2,[[]]],[11,"not_equal_bm","","",2,[[]]],[11,"equal_bm","","",2,[[]]],[11,"greater_than_bm","","",2,[[]]],[11,"greater_than_or_equal_bm","","",2,[[]]],[11,"less_than_bm","","",2,[[]]],[11,"less_than_or_equal_bm","","",2,[[]]],[11,"max_val","","Return largest value that can be represented.",3,[[]]],[11,"wrap_add","","",3,[[]]],[11,"wrap_sub","","",3,[[]]],[11,"wrap_mul","","",3,[[]]],[11,"wrap_div","","",3,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",3,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",3,[[]]],[11,"divide","","Division.",3,[[]]],[11,"inv","","Invert self modulo n.",3,[[]]],[11,"equal","","",3,[[]]],[11,"greater_than","","",3,[[]]],[11,"greater_than_or_qual","","",3,[[]]],[11,"less_than","","",3,[[]]],[11,"less_than_or_equal","","",3,[[]]],[11,"not_equal_bm","","",3,[[]]],[11,"equal_bm","","",3,[[]]],[11,"greater_than_bm","","",3,[[]]],[11,"greater_than_or_equal_bm","","",3,[[]]],[11,"less_than_bm","","",3,[[]]],[11,"less_than_or_equal_bm","","",3,[[]]]],"p":[[3,"State"],[3,"StateBytes"],[3,"IV"],[3,"Key"]]},\
"chacha20poly1305":{"doc":"","i":[[5,"encrypt","chacha20poly1305","",null,[[["key",3],["byteseq",6],["iv",3]]]],[5,"decrypt","","",null,[[["key",3],["byteseq",6],["tag",3],["iv",3]]]]],"p":[]},\
"chacha20poly1305_rfc7539":{"doc":"ChaCha20Poly1305 is specified in RFC 7539.","i":[[5,"encrypt","chacha20poly1305_rfc7539","",null,[[["key",3],["byteseq",6],["iv",3]],[["string",3],["result",4]]]],[5,"decrypt","","",null,[[["key",3],["byteseq",6],["tag",3],["iv",3]],[["byteseq",6],["string",3],["result",4]]]]],"p":[]},\
"main":{"doc":"This is a demo of hacspec for the HACS workshop on…","i":[],"p":[]},\
"poly1305":{"doc":"","i":[[3,"Block","poly1305","Fixed length byte array.",null,null],[12,"0","","",0,null],[3,"Tag","","Fixed length byte array.",null,null],[12,"0","","",1,null],[3,"FieldCanvas","","",null,null],[3,"FieldElement","","",null,null],[5,"poly","","",null,[[["key",3],["byteseq",6]],["tag",3]]],[5,"poly_mac","","",null,[[["key",3],["byteseq",6],["iv",3]],["tag",3]]],[11,"new","","",0,[[]]],[11,"length","","",0,[[]]],[11,"from_array","","",0,[[]]],[11,"from_native_slice","","",0,[[]]],[11,"capacity","","",0,[[]]],[11,"from_slice","","",0,[[]]],[11,"from_slice_range","","",0,[[["range",3]]]],[11,"slice","","",0,[[],[["u8",3],["seq",3]]]],[11,"slice_range","","",0,[[["range",3]],[["u8",3],["seq",3]]]],[11,"num_chunks","","",0,[[]]],[11,"get_chunk_len","","",0,[[]]],[11,"get_chunk","","",0,[[]]],[11,"set_chunk","","",0,[[]]],[11,"from_vec","","",0,[[["vec",3],["u8",3]],["block",3]]],[11,"from_seq","","",0,[[],["block",3]]],[11,"from_hex","","Read hex string to Bytes.",0,[[],["block",3]]],[11,"declassify_eq","","",0,[[]]],[11,"to_be_bytes","","",0,[[],[["u8",3],["seq",3]]]],[11,"to_le_bytes","","",0,[[],[["u8",3],["seq",3]]]],[11,"from_public_slice","","",0,[[],["block",3]]],[11,"from_public_array","","Create an array from a regular Rust array.",0,[[],["block",3]]],[11,"to_be_U32s","","",0,[[],[["u32",3],["seq",3]]]],[11,"to_le_U32s","","",0,[[],[["u32",3],["seq",3]]]],[11,"to_be_U64s","","",0,[[],[["seq",3],["u64",3]]]],[11,"to_le_U64s","","",0,[[],[["seq",3],["u64",3]]]],[11,"to_U128s_be","","",0,[[],[["u128",3],["seq",3]]]],[11,"to_U128s_le","","",0,[[],[["u128",3],["seq",3]]]],[11,"to_hex","","",0,[[],["string",3]]],[11,"new","","",1,[[]]],[11,"length","","",1,[[]]],[11,"from_array","","",1,[[]]],[11,"from_native_slice","","",1,[[]]],[11,"capacity","","",1,[[]]],[11,"from_slice","","",1,[[]]],[11,"from_slice_range","","",1,[[["range",3]]]],[11,"slice","","",1,[[],["seq",3]]],[11,"slice_range","","",1,[[["range",3]],["seq",3]]],[11,"num_chunks","","",1,[[]]],[11,"get_chunk_len","","",1,[[]]],[11,"get_chunk","","",1,[[]]],[11,"set_chunk","","",1,[[]]],[11,"from_vec","","",1,[[["vec",3]],["tag",3]]],[11,"from_seq","","",1,[[],["tag",3]]],[11,"from_hex","","Read hex string to Bytes.",1,[[],["tag",3]]],[11,"to_be_u32s","","",1,[[],["seq",3]]],[11,"to_le_u32s","","",1,[[],["seq",3]]],[11,"to_be_u64s","","",1,[[],["seq",3]]],[11,"to_le_u64s","","",1,[[],["seq",3]]],[11,"to_u128s_be","","",1,[[],["seq",3]]],[11,"to_u128s_le","","",1,[[],["seq",3]]],[11,"to_hex","","",1,[[],["string",3]]],[11,"max_value","","",2,[[]]],[11,"from_literal","","",2,[[]]],[11,"from_signed_literal","","",2,[[]]],[11,"pow2","","Returns 2 to the power of the argument",2,[[],["fieldcanvas",3]]],[11,"bit","","Gets the `i`-th least significant bit of this integer.",2,[[]]],[11,"from_hex","","",2,[[]]],[11,"from_le_bytes","","",2,[[]]],[11,"to_le_bytes","","",2,[[],["vec",3]]],[11,"comp_eq","","Produces a new integer which is all ones if the two…",2,[[]]],[11,"comp_ne","","Produces a new integer which is all ones if the first…",2,[[]]],[11,"comp_gte","","Produces a new integer which is all ones if the first…",2,[[]]],[11,"comp_gt","","Produces a new integer which is all ones if the first…",2,[[]]],[11,"comp_lte","","Produces a new integer which is all ones if the first…",2,[[]]],[11,"comp_lt","","Produces a new integer which is all ones if the first…",2,[[]]],[11,"inv","","",2,[[]]],[11,"pow_felem","","",2,[[]]],[11,"pow","","Returns self to the power of the argument. The exponent is…",2,[[]]],[11,"from_canvas","","",3,[[["fieldcanvas",3]],["fieldelement",3]]],[11,"into_canvas","","",3,[[],["fieldcanvas",3]]],[11,"max","","",3,[[],["fieldcanvas",3]]],[11,"declassify","","",3,[[],["bigint",3]]],[11,"from_hex","","",3,[[]]],[11,"from_le_bytes","","",3,[[]]],[11,"to_le_bytes","","",3,[[],["vec",3]]],[11,"bit","","Gets the `i`-th least significant bit of this integer.",3,[[]]],[11,"from_literal","","",3,[[]]],[11,"from_signed_literal","","",3,[[]]],[11,"comp_eq","","",3,[[]]],[11,"comp_ne","","",3,[[]]],[11,"comp_gte","","",3,[[]]],[11,"comp_gt","","",3,[[]]],[11,"comp_lte","","",3,[[]]],[11,"comp_lt","","",3,[[]]],[11,"inv","","",3,[[]]],[11,"pow_felem","","",3,[[]]],[11,"pow","","Returns self to the power of the argument. The exponent is…",3,[[]]],[11,"from_byte_seq_le","","",3,[[["seqtrait",8],["u8",3]],["fieldelement",3]]],[11,"to_public_byte_seq_le","","",3,[[],["seq",3]]],[11,"to_byte_seq_le","","",3,[[],[["u8",3],["seq",3]]]],[11,"from_secret_literal","","",3,[[["u128",3]],["fieldelement",3]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"to_string","","",2,[[],["string",3]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",2,[[["biguint",3]],["fieldcanvas",3]]],[11,"from","","",2,[[["bigint",3]],["fieldcanvas",3]]],[11,"from","","",3,[[["fieldcanvas",3]],["fieldelement",3]]],[11,"into","","",2,[[],["bigint",3]]],[11,"into","","",2,[[],["biguint",3]]],[11,"into","","",3,[[],["fieldcanvas",3]]],[11,"clone","","",0,[[],["block",3]]],[11,"clone","","",1,[[],["tag",3]]],[11,"clone","","",2,[[],["fieldcanvas",3]]],[11,"clone","","",3,[[],["fieldelement",3]]],[11,"default","","",0,[[]]],[11,"default","","",1,[[]]],[11,"default","","",2,[[],["fieldcanvas",3]]],[11,"default","","",3,[[],["fieldelement",3]]],[11,"cmp","","",1,[[],["ordering",4]]],[11,"cmp","","",2,[[["fieldcanvas",3]],["ordering",4]]],[11,"cmp","","",3,[[],["ordering",4]]],[11,"eq","","",1,[[]]],[11,"eq","","",2,[[["fieldcanvas",3]]]],[11,"eq","","",3,[[]]],[11,"partial_cmp","","",1,[[],[["option",4],["ordering",4]]]],[11,"partial_cmp","","",2,[[["fieldcanvas",3]],[["option",4],["ordering",4]]]],[11,"partial_cmp","","",3,[[],[["option",4],["ordering",4]]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"div","","",1,[[["tag",3]],["tag",3]]],[11,"div","","",2,[[["fieldcanvas",3]],["fieldcanvas",3]]],[11,"div","","",3,[[["fieldelement",3]],["fieldelement",3]]],[11,"rem","","",0,[[["block",3]],["block",3]]],[11,"rem","","",2,[[["fieldcanvas",3]],["fieldcanvas",3]]],[11,"rem","","",3,[[["fieldelement",3]],["fieldelement",3]]],[11,"sub","","",0,[[["block",3]],["block",3]]],[11,"sub","","",1,[[["tag",3]],["tag",3]]],[11,"sub","","",2,[[["fieldcanvas",3]],["fieldcanvas",3]]],[11,"sub","","",3,[[["fieldelement",3]],["fieldelement",3]]],[11,"add","","",0,[[["block",3]],["block",3]]],[11,"add","","",1,[[["tag",3]],["tag",3]]],[11,"add","","",2,[[["fieldcanvas",3]],["fieldcanvas",3]]],[11,"add","","",3,[[["fieldelement",3]],["fieldelement",3]]],[11,"mul","","",0,[[["block",3]],["block",3]]],[11,"mul","","",1,[[["tag",3]],["tag",3]]],[11,"mul","","",2,[[["fieldcanvas",3]],["fieldcanvas",3]]],[11,"mul","","",3,[[["fieldelement",3]],["fieldelement",3]]],[11,"not","","",0,[[]]],[11,"not","","",1,[[]]],[11,"not","","",2,[[]]],[11,"not","","",3,[[]]],[11,"bitand","","",0,[[]]],[11,"bitand","","",1,[[]]],[11,"bitand","","",2,[[]]],[11,"bitand","","",3,[[]]],[11,"bitor","","",0,[[]]],[11,"bitor","","",1,[[]]],[11,"bitor","","",2,[[]]],[11,"bitor","","",3,[[]]],[11,"bitxor","","",0,[[]]],[11,"bitxor","","",1,[[]]],[11,"bitxor","","",2,[[]]],[11,"bitxor","","",3,[[]]],[11,"shl","","",0,[[]]],[11,"shl","","",1,[[]]],[11,"shl","","",2,[[]]],[11,"shl","","",3,[[]]],[11,"shr","","",0,[[]]],[11,"shr","","",1,[[]]],[11,"shr","","",2,[[]]],[11,"shr","","",3,[[]]],[11,"index","","",0,[[],["u8",3]]],[11,"index","","",0,[[],["u8",3]]],[11,"index","","",0,[[],["u8",3]]],[11,"index","","",0,[[],["u8",3]]],[11,"index","","",0,[[["rangefull",3]]]],[11,"index","","",1,[[]]],[11,"index","","",1,[[]]],[11,"index","","",1,[[]]],[11,"index","","",1,[[]]],[11,"index","","",1,[[["rangefull",3]]]],[11,"index_mut","","",0,[[],["u8",3]]],[11,"index_mut","","",0,[[],["u8",3]]],[11,"index_mut","","",0,[[],["u8",3]]],[11,"index_mut","","",0,[[],["u8",3]]],[11,"index_mut","","",1,[[]]],[11,"index_mut","","",1,[[]]],[11,"index_mut","","",1,[[]]],[11,"index_mut","","",1,[[]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"create","","",0,[[]]],[11,"len","","",0,[[]]],[11,"iter","","",0,[[],[["u8",3],["iter",3]]]],[11,"update_slice","","",0,[[]]],[11,"update","","",0,[[]]],[11,"update_start","","",0,[[]]],[11,"create","","",1,[[]]],[11,"len","","",1,[[]]],[11,"iter","","",1,[[],["iter",3]]],[11,"update_slice","","",1,[[]]],[11,"update","","",1,[[]]],[11,"update_start","","",1,[[]]],[11,"ZERO","","",2,[[]]],[11,"ONE","","",2,[[]]],[11,"TWO","","",2,[[]]],[11,"from_literal","","",2,[[]]],[11,"from_hex_string","","",2,[[["string",3]]]],[11,"get_bit","","Get bit `i` of this integer.",2,[[]]],[11,"set_bit","","Set bit `i` of this integer to `b` and return the result.…",2,[[]]],[11,"set","","Set bit `pos` of this integer to bit `yi` of integer `y`.",2,[[]]],[11,"rotate_left","","",2,[[]]],[11,"rotate_right","","",2,[[]]],[11,"ZERO","","",3,[[]]],[11,"ONE","","",3,[[]]],[11,"TWO","","",3,[[]]],[11,"from_literal","","",3,[[]]],[11,"from_hex_string","","",3,[[["string",3]]]],[11,"get_bit","","Get bit `i` of this integer.",3,[[]]],[11,"set_bit","","Set bit `i` of this integer to `b` and return the result.…",3,[[]]],[11,"set","","Set bit `pos` of this integer to bit `yi` of integer `y`.",3,[[]]],[11,"rotate_left","","",3,[[]]],[11,"rotate_right","","",3,[[]]],[11,"sub_mod","","(self - rhs) % n.",0,[[]]],[11,"add_mod","","`(self + rhs) % n`",0,[[]]],[11,"mul_mod","","`(self * rhs) % n`",0,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",0,[[]]],[11,"modulo","","`self % n`",0,[[]]],[11,"signed_modulo","","",0,[[]]],[11,"absolute","","`|self|` (coefficient-wise)",0,[[]]],[11,"sub_mod","","`(self - rhs) % n` (coefficient-wise)",1,[[]]],[11,"add_mod","","`(self + rhs) % n` (coefficient-wise)",1,[[]]],[11,"mul_mod","","`(self * rhs) % n` (coefficient-wise) Note that the…",1,[[]]],[11,"pow_mod","","`(self ^ exp) % n` (coefficient-wise) Note that the…",1,[[]]],[11,"modulo","","`self % n` (coefficient-wise)",1,[[]]],[11,"signed_modulo","","`self % n` (coefficient-wise)",1,[[]]],[11,"absolute","","`|self|` (coefficient-wise)",1,[[]]],[11,"sub_mod","","(self - rhs) % n.",2,[[]]],[11,"add_mod","","`(self + rhs) % n`",2,[[]]],[11,"mul_mod","","`(self * rhs) % n`",2,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",2,[[]]],[11,"modulo","","`self % n`",2,[[]]],[11,"signed_modulo","","`self % n` that always returns a positive integer",2,[[]]],[11,"absolute","","`|self|`",2,[[]]],[11,"sub_mod","","(self - rhs) % n.",3,[[]]],[11,"add_mod","","`(self + rhs) % n`",3,[[]]],[11,"mul_mod","","`(self * rhs) % n`",3,[[]]],[11,"pow_mod","","`(self ^ exp) % n`",3,[[]]],[11,"modulo","","`self % n`",3,[[]]],[11,"signed_modulo","","`self % n` that always returns a positive integer",3,[[]]],[11,"absolute","","`|self|`",3,[[]]],[11,"max_val","","Return largest value that can be represented.",0,[[]]],[11,"wrap_add","","",0,[[]]],[11,"wrap_sub","","",0,[[]]],[11,"wrap_mul","","",0,[[]]],[11,"wrap_div","","",0,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",0,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",0,[[]]],[11,"divide","","Division.",0,[[]]],[11,"inv","","Invert self modulo n.",0,[[]]],[11,"equal","","",0,[[]]],[11,"greater_than","","",0,[[]]],[11,"greater_than_or_qual","","",0,[[]]],[11,"less_than","","",0,[[]]],[11,"less_than_or_equal","","",0,[[]]],[11,"not_equal_bm","","",0,[[]]],[11,"equal_bm","","",0,[[]]],[11,"greater_than_bm","","",0,[[]]],[11,"greater_than_or_equal_bm","","",0,[[]]],[11,"less_than_bm","","",0,[[]]],[11,"less_than_or_equal_bm","","",0,[[]]],[11,"max_val","","Return largest value that can be represented. Not…",1,[[]]],[11,"wrap_add","","`self + rhs` (coefficient-wise and wrapping)",1,[[]]],[11,"wrap_sub","","`self - rhs` (coefficient-wise and wrapping)",1,[[]]],[11,"wrap_mul","","`self * rhs` (coefficient-wise and wrapping)",1,[[]]],[11,"wrap_div","","`self + rhs` (coefficient-wise and wrapping)",1,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32` (coefficient-wise and…",1,[[]]],[11,"pow_self","","Not implemented.",1,[[]]],[11,"divide","","`self / rhs` (coefficient-wise and wrapping).",1,[[]]],[11,"inv","","Not implemented",1,[[]]],[11,"equal","","",1,[[]]],[11,"greater_than","","",1,[[]]],[11,"greater_than_or_qual","","",1,[[]]],[11,"less_than","","",1,[[]]],[11,"less_than_or_equal","","",1,[[]]],[11,"not_equal_bm","","",1,[[]]],[11,"equal_bm","","",1,[[]]],[11,"greater_than_bm","","",1,[[]]],[11,"greater_than_or_equal_bm","","",1,[[]]],[11,"less_than_bm","","",1,[[]]],[11,"less_than_or_equal_bm","","",1,[[]]],[11,"max_val","","Return largest value that can be represented.",2,[[]]],[11,"wrap_add","","",2,[[]]],[11,"wrap_sub","","",2,[[]]],[11,"wrap_mul","","",2,[[]]],[11,"wrap_div","","",2,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",2,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",2,[[]]],[11,"divide","","Division.",2,[[]]],[11,"inv","","Invert self modulo n.",2,[[]]],[11,"equal","","",2,[[]]],[11,"greater_than","","",2,[[]]],[11,"greater_than_or_qual","","",2,[[]]],[11,"less_than","","",2,[[]]],[11,"less_than_or_equal","","",2,[[]]],[11,"not_equal_bm","","",2,[[]]],[11,"equal_bm","","",2,[[]]],[11,"greater_than_bm","","",2,[[]]],[11,"greater_than_or_equal_bm","","",2,[[]]],[11,"less_than_bm","","",2,[[]]],[11,"less_than_or_equal_bm","","",2,[[]]],[11,"max_val","","Return largest value that can be represented.",3,[[]]],[11,"wrap_add","","",3,[[]]],[11,"wrap_sub","","",3,[[]]],[11,"wrap_mul","","",3,[[]]],[11,"wrap_div","","",3,[[]]],[11,"exp","","`self ^ exp` where `exp` is a `u32`.",3,[[]]],[11,"pow_self","","`self ^ exp` where `exp` is a `Self`.",3,[[]]],[11,"divide","","Division.",3,[[]]],[11,"inv","","Invert self modulo n. NOTE: `n` is ignored and inversion…",3,[[]]],[11,"equal","","",3,[[]]],[11,"greater_than","","",3,[[]]],[11,"greater_than_or_qual","","",3,[[]]],[11,"less_than","","",3,[[]]],[11,"less_than_or_equal","","",3,[[]]],[11,"not_equal_bm","","",3,[[]]],[11,"equal_bm","","",3,[[]]],[11,"greater_than_bm","","",3,[[]]],[11,"greater_than_or_equal_bm","","",3,[[]]],[11,"less_than_bm","","",3,[[]]],[11,"less_than_or_equal_bm","","",3,[[]]]],"p":[[3,"Block"],[3,"Tag"],[3,"FieldCanvas"],[3,"FieldElement"]]},\
"poly_demo":{"doc":"","i":[[5,"round_to_3","poly_demo","First transform each coefficients to a value between…",null,[[["seq",3]],["seq",3]]],[5,"ntru_prime_653_encrypt","","NTRU Prime 653 basic encryption",null,[[["seq",3]],["seq",3]]],[5,"ntru_prime_653_decrypt","","NTRU Prime 653 basic decryption",null,[[["seq",3]],["seq",3]]]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);