# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_float_load, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED for encode_float_load. The cargo test run executed the Rust symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_float_load (float.rs:5) |
| Properties | 8 (5 passing, 3 failing) |
| KAT | 4 passing |
| Regression witnesses | 3 failing (intentional) |
| Bugs | 3 |
| Sweep | 1/1 spent (no line-level data; documented branches covered by properties) |

## Documented behaviors vs properties

| Behavior | Property | Result |
|----------|----------|--------|
| Valid Mem FLW/FLD vs llvm-mc | encode_float_load_diff_imm_llvm_mc | passing |
| I-type field unpack OP_LOAD_FP | encode_float_load_i_type_fields | passing |
| FP ABI vs fN / GPR ABI vs xN / fp=s0 | encode_float_load_abi_fn_alias | passing |
| %lo/%pcrel_lo/%tprel_lo reloc-form | encode_float_load_reloc_lo | passing |
| imm outside [-2048, 2047] must Err | encode_float_load_neg_imm_oob | failing (B1) |
| extra operand must Err | encode_float_load_neg_extra | failing (B2) |
| empty / missing / GPR dest / FP base / non-mem 2nd | encode_float_load_neg_arity_gpr | passing |
| %hi/%pcrel_hi/%tprel_hi must Err | encode_float_load_neg_hi_modifier | failing (B3) |
