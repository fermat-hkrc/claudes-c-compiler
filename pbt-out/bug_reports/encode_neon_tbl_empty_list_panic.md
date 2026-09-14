# Bug: encode_neon_tbl panics on an empty register list
**Law:** An invalid table list must return Err. The encoder returns `Result<EncodeResult, String>` and must not panic on an Operand the API accepts.
**Impact:** `Operand::RegList(vec![])` indexes `regs[0]` and panics (`index out of bounds: the len is 0 but the index is 0`). A `Result`-returning encoder that panics on bad operands can abort assembly of otherwise valid files if a list is empty.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, rm=0, kind=0 — operands `[RegArrangement{v0,"8b"}, RegList([]), RegArrangement{v0,"8b"}]`
**Expected:** Err
**Actual:** panic at neon.rs:781 `regs[0]`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_empty_list
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_tbl -- --test-threads=1`
