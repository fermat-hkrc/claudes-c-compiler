# Bug: encode_neon_qshrn accepts a GPR/FP dest as if it were Vd
**Law:** Vector SQSHRN dest must be a NEON register with arrangement (Vd.Tb). A GPR (x/w) or scalar FP (d/s/q/h/b) dest must be rejected.
**Impact:** Callers `uqshrn` / `sqshrn2` / `sqrshrn` / `uqrshrn` (+2) dispatch unconditionally to this helper (encoder/mod.rs:642-648). `uqshrn x0, v0.8h, #1` is invalid AArch64 but encodes as Rd=0 (same as v0). Silent wrong encoding of a GPR dest.
**Function:** encode_neon_qshrn
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** dest=x0 (Operand::Reg), rn=0, ta=8h, shift=1, u_bit=0, is_rounding=false, is_high=false (`sqshrn x0, v0.8h, #1` against the helper; reachable as `uqshrn`/`sqshrn2`/…)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `get_neon_reg` accepts `Operand::Reg` via `parse_reg_num`, which maps x/w/d/s/q/h/b prefixes to 0..31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_qshrn_regression_gpr_dest
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_qshrn_neg -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- README.md:5-14 gas-compatible GNU-style assembly
- llvm-mc rejects `sqshrn x0, v1.8h, #1`
- ARM ARM SQSHRN dest is Vd.Tb
- Dispatch: sqshrn2/uqshrn/sqrshrn/uqrshrn (+2) call encode_neon_qshrn without a RegArrangement gate
