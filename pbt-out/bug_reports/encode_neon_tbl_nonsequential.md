# Bug: encode_neon_tbl ignores non-sequential table registers
**Law:** Table registers must be consecutive, wrapping at 31 (llvm-mc: "registers must be sequential"). A gap such as `{v0.16b, v2.16b}` must be rejected with Err.
**Impact:** Only the first register number and the list length are encoded. `{v0.16b, v2.16b}` is emitted as a 2-register table starting at v0 (i.e. `{v0.16b, v1.16b}`), silently changing the table.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, list=`{v0.16b, v2.16b}`, rm=0 — `tbl v0.8b, {v0.16b, v2.16b}, v0.8b`
**Expected:** Err
**Actual:** Ok(Word(...)) encoding Rn=0, len=01 (as if `{v0.16b, v1.16b}`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_nonsequential
**Serial reconfirmation:** SUT matches only `regs[0]` then uses `regs.len()`; same serial run as encode_neon_tbl_neg_table_contract (kind=2 in generator)
