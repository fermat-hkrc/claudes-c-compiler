# Bug: encode_neon_sli ignores source arrangement
**Law:** Vector SLI requires matching arrangements: `Vd.T, Vn.T, #shift` with the same T. A dest/src T mismatch must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `sli v0.8b, v0.16b, #0` is silently encoded using only the destination T, so the built-in assembler accepts text that gas/llvm-mc reject (`invalid operand for instruction`).
**Function:** encode_neon_sli
**Detected by:** Negative/Error Contract (4e)
**Minimal input:**
- rd=0, rn=0, td="8b", tn="16b", shift=0 — `sli v0.8b, v0.16b, #0` (operands `[RegArrangement{v0,"8b"}, RegArrangement{v0,"16b"}, Imm(0)]`)
- coverage sweep: `[RegArrangement{v0,"8b"}, Reg("v0"), Imm(0)]` — `sli v0.8b, v0, #0` (llvm-mc rejects missing source T)
**Expected:** Err
**Actual:** Ok(Word(...)) — `let (rn, _) = get_neon_reg(operands, 1)` discards the source arrangement (including empty arrangement from Operand::Reg); only dest T is used
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_sli_pbt::test_encode_neon_sli_regression_arrangement_mismatch and test_encode_neon_sli_regression_src_reg_no_arrangement
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_sli_neg -- --test-threads=1`
