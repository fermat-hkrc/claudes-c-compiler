# Bug: encode_neon_shll accepts a GPR/FP dest as if it were Vd.Ta
**Law:** SSHLL dest is a NEON arrangement register Vd.Ta. A GPR or scalar FP dest (`x0`, `w0`, `d0`, ...) must be rejected.
**Impact:** `sshll x0, v0.8b, #0` is invalid AArch64 (llvm-mc: invalid operand) but `get_neon_reg` accepts `Operand::Reg` and `parse_reg_num` maps `x0`/`w0`/`d0` to register 0, so the helper encodes as `v0`. Wrong-class dest is silently accepted.
**Function:** encode_neon_shll
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** prefix=x, n=0, rn=0, tb=8b, shift=0, u_bit=0 (`sshll x0, v0.8b, #0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) encoding Rd=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_shll_regression_gpr_dest
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_shll_neg_gpr -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- ARM ARM SSHLL syntax: Vd.Ta, Vn.Tb, #shift
- llvm-mc rejects `sshll x0, v1.8b, #0`
