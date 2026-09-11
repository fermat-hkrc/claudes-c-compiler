# Bug: encode_adc accepts FP/SIMD register names as GPRs
**Law:** ARM ADC operands are W/X general-purpose registers only. llvm-mc rejects `adc d0, d1, d2` and `adc d0, x1, x2`. encode_adc must Err when any operand is a d/s/q/v/h/b register.
**Impact:** `adc d0, x1, x2` is encoded as 32-bit `adc w0, x1, x2` (parse_reg_num maps d0→0, is_64bit_reg is false). The assembler silently produces an integer ADC instead of rejecting illegal FP operands.
**Function:** encode_adc
**Detected by:** Algebraic — Negative/Error Contract (4e); coverage-sweep of parse_reg_num FP prefixes
**Minimal input:** operands = [Reg("d0"), Reg("x1"), Reg("x2")], set_flags = false
**Expected:** Err (invalid operand; ADC is integer data-processing)
**Actual:** Ok(Word) — treated as w0 (sf=0, rd=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_adc_pbt::test_encode_adc_regression_fp_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adc_neg_fp_reg -- --test-threads=1)
