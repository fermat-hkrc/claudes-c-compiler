# Bug: encode_neon_tbl accepts a GPR/FP destination
**Law:** TBL destination is `Vd.Ta` (a NEON register with arrangement 8B or 16B). A bare GPR/FP name (`x0`, `w0`, `d0`, …) must be rejected (llvm-mc: invalid operand).
**Impact:** `tbl x0, {v0.16b}, v0.8b` is encoded with Rd=0 and Q=0 (empty arrangement is treated as not-16b), producing a TBL to v0.8b from a GPR dest token.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** n=0, prefix="x", dest_n=0 — operands `[Reg("x0"), RegList({v0.16b}), RegArrangement{v0,"8b"}]` corresponding to `tbl x0, {v0.16b}, v0.8b`
**Expected:** Err
**Actual:** Ok(Word(...)) — `get_neon_reg` accepts `Operand::Reg` and returns an empty arrangement
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_gpr_dest
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_tbl -- --test-threads=1`
