# Bug: encode_adr accepts FP/SIMD register names as GPR Rd
**Law:** ARM ADR operands are 64-bit X registers only. llvm-mc rejects `adr d0, #0`. encode_adr must Err when Rd is a d/s/q/v/h/b register.
**Impact:** `adr d0, #0` is encoded as `adr x0, #0` (0x10000000) because parse_reg_num maps d0→0 and encode_adr ignores the is_64 flag. The assembler silently produces an integer ADR instead of rejecting illegal FP operands.
**Function:** encode_adr
**Detected by:** Algebraic — Negative/Error Contract (4e); coverage-sweep of parse_reg_num FP prefixes
**Minimal input:** operands = [Reg("d0"), Imm(-1048576)]
**Expected:** Err (invalid operand; ADR is not an FP instruction)
**Actual:** Ok(Word) — treated as x0 (rd=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_adr_pbt::test_encode_adr_regression_fp_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adr_neg_fp_reg -- --test-threads=1)
