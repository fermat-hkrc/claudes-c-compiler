# Bug: encode_neon_tbl accepts a bare Reg as the table list member
**Law:** ARM ARM table registers are `Vn.16B`. llvm-mc rejects `{v0}` without an arrangement. A list member that is `Operand::Reg` (no arrangement) must Err.
**Impact:** `Operand::RegList([Reg("v0")])` encodes as a 1-register TBL (arrangement is never consulted), so `{v0}` is treated as `{v0.16b}`.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e) — coverage sweep of neon.rs:780-784
**Minimal input:** rd=0, rn=0, rm=0, kind=0 — operands `[RegArrangement{v0,"8b"}, RegList([Reg("v0")]), RegArrangement{v0,"8b"}]` → Ok(Word(0x0e000000))
**Expected:** Err
**Actual:** Ok(Word(0x0e000000)) — `match &regs[0] { Operand::Reg(name) => parse_reg_num(name) }` accepts a bare register
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_bare_list_reg
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_tbl_neg_list_and_vm_kinds -- --test-threads=1`
