# Bug: ADD/SUB encodes FP/SIMD register names as GPRs
**Law:** ARM ARM ADD/SUB (data-processing) operands are W/X registers. llvm-mc rejects `add d0, x1, x2` / `adds d0, d1, d2`. A gas-compatible assembler must Err. (NEON vector ADD uses RegArrangement `vN.T`, a different form already handled.)
**Impact:** `add d0, x1, x2` is encoded as `add w0, x1, x2` (parse_reg_num accepts the `d` prefix). `adds d0, …` is caller-reachable: encode_instruction dispatches adds/subs to encode_add_sub with no neon scalar filter.
**Function:** encode_add_sub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1"), Reg("x2")], is_sub=false, set_flags=false
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps d0→0 and is_64bit_reg is false, so it encodes as W0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_fp_reg
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_add_sub_neg_fp_reg -- --test-threads=1`
