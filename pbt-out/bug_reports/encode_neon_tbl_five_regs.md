# Bug: encode_neon_tbl wraps table length above 4 instead of rejecting
**Law:** ARM ARM TBL allows 1 to 4 table registers (`len` in {00,01,10,11}). Five or more vectors must be rejected (llvm-mc: "invalid number of vectors").
**Impact:** A 5-register list encodes as `len = (5-1) & 0x3 = 0` (the 1-register encoding). `tbl v0.8b, {v0.16b, v1.16b, v2.16b, v3.16b, v4.16b}, v0.8b` silently becomes a 1-register TBL, dropping four table registers.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, rm=0, n=5 — `tbl v0.8b, {v0.16b, v1.16b, v2.16b, v3.16b, v4.16b}, v0.8b`
**Expected:** Err
**Actual:** Ok(Word(...)) with len=00 — `let len = (num_regs - 1) & 0x3` wraps counts above 4
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_five_regs
**Serial reconfirmation:** SUT body `len = (num_regs - 1) & 0x3`; same serial run as encode_neon_tbl_neg_table_contract (kind=1 in generator)
