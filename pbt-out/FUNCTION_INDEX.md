> FUNCTION_INDEX — campaign scope: src/common/{encoding,const_arith,types}.rs + E2E driver surface.
> Total files: 4 | Total functions: 43 | PBT candidates: 8 | Excluded: 35

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| bytes_to_string | encoding.rs | 22 | function | yes | PUA encode contract (P6/P7) |
| encode_non_utf8 | encoding.rs | 37 | function | no | private, covered via bytes_to_string |
| utf8_sequence_length | encoding.rs | 65 | function | no | private helper |
| decode_pua_byte | encoding.rs | 80 | function | yes | PUA decode contract (P6) |
| wrap_result | const_arith.rs | 19 | function | no | private, covered via eval_const_binop |
| unsigned_op | const_arith.rs | 27 | function | no | private helper |
| bool_to_i64 | const_arith.rs | 37 | function | no | trivial |
| eval_const_binop_int | const_arith.rs | 54 | function | no | private, covered via eval_const_binop |
| eval_const_binop_float | const_arith.rs | 150 | function | no | float folding (out of budget; E2E covers float pass-through) |
| eval_const_binop_i128 | const_arith.rs | 310 | function | no | private, i128 path (out of budget) |
| eval_const_binop | const_arith.rs | 392 | function | yes | int const-fold contract (P8) |
| negate_const | const_arith.rs | 413 | function | no | trivial wrapping ops |
| bitnot_const | const_arith.rs | 435 | function | no | trivial |
| is_zero_expr | const_arith.rs | 448 | function | no | AST predicate |
| is_null_pointer_constant | const_arith.rs | 467 | function | no | AST predicate |
| is_integer_constant_expr_zero | const_arith.rs | 494 | function | no | private AST predicate |
| is_syntactically_constant | const_arith.rs | 506 | function | no | private AST predicate |
| is_zero_valued_constant | const_arith.rs | 552 | function | no | private AST predicate |
| truncate_and_extend_bits | const_arith.rs | 581 | function | no | bit helper, exercised via P8 path |
| set_target_ptr_size | types.rs | 22 | function | no | global setter (thread-unsafe to test) |
| target_ptr_size | types.rs | 27 | function | no | trivial getter |
| target_is_32bit | types.rs | 32 | function | no | trivial getter |
| set_target_long_double_is_f128 | types.rs | 37 | function | no | global setter |
| target_long_double_is_f128 | types.rs | 43 | function | no | trivial getter |
| target_int_ir_type | types.rs | 52 | function | no | trivial constructor |
| widened_op_type | types.rs | 64 | function | no | trivial mapping |
| EightbyteClass::merge | types.rs | 125 | method | no | ABI class merge (covered by E2E P4 struct-by-value) |
| EnumType::packed_size | types.rs | 256 | method | no | enum size (covered by E2E P3 sizeof) |
| StructLayout::empty | types.rs | 517 | method | no | constructor |
| StructLayout::empty_union | types.rs | 528 | method | no | constructor |
| StructLayout::empty_rc | types.rs | 539 | method | no | constructor |
| StructLayout::empty_union_rc | types.rs | 544 | method | no | constructor |
| StructLayout::for_struct_with_packing | types.rs | 602 | method | no | covered differentially via E2E P3 (sizeof/offsetof vs gcc) |
| StructLayout::for_union_with_packing | types.rs | 629 | method | no | covered via E2E P3 union bytes |
| StructLayout::has_pointer_fields | types.rs | 685 | method | no | predicate |
| StructLayout::classify_sysv_eightbytes | types.rs | 714 | method | no | covered via E2E P4 struct-by-value calls |
| StructLayout::classify_riscv_float_fields | types.rs | 848 | method | no | RISC-V only (skipped module) |
| StructLayout::resolve_init_field_idx | types.rs | 1016 | method | no | initializer resolution (covered via E2E P5) |
| StructLayout::resolve_init_field | types.rs | 1029 | method | no | initializer resolution |
| StructLayout::field_offset | types.rs | 1097 | method | no | covered via E2E P3 offsetof |
| StructLayout::field_layout | types.rs | 1136 | method | no | accessor |
| StructLayout::field_offset_with_bitfield | types.rs | 1144 | method | no | covered via E2E P3 bitfield values |
| align_up | types.rs | 1177 | function | yes | alignment law (P9) |
| compiler_main (E2E pipeline) | src/lib.rs | 8 | function | yes | full-pipeline differential (P1-P5) |

Note: E2E properties P1-P5 exercise the whole driver→codegen→linker pipeline through
the `ccc` binary; per-function coverage of backend internals is reported by the
campaign's native coverage data (see pbt-out/code-coverage).
