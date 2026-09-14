# Bug: encode_neon_tbl accepts a table arrangement other than .16B
**Law:** ARM ARM table registers are `.16B`. llvm-mc rejects `{v0.8b}` as an invalid operand. The table arrangement must be 16b or the encoder must Err.
**Impact:** `tbl v0.8b, {v0.8b}, v0.8b` is encoded as a 1-register TBL (arrangement of list members is ignored), so the assembler accepts GNU-invalid text.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, table=`{v0.8b}`, rm=0 — `tbl v0.8b, {v0.8b}, v0.8b`
**Expected:** Err
**Actual:** Ok(Word(...)) — only the first list register's number is parsed; its arrangement is discarded
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_table_not_16b
**Serial reconfirmation:** SUT `match &regs[0]` reads only the name; same serial run as encode_neon_tbl_neg_table_contract (kind=3 in generator)
