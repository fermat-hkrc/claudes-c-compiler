# Bug: encode_neon_tbl accepts a bare GPR/FP/V as Vm
**Law:** TBL's index register is `Vm.Ta`. llvm-mc rejects `tbl v0.8b, {v0.16b}, x0`. A bare `Operand::Reg` in the Vm slot must Err.
**Impact:** `get_neon_reg` accepts `Operand::Reg`, so `tbl v0.8b, {v0.16b}, x0` encodes Rm from the GPR number and ignores the missing arrangement.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e) — coverage sweep of get_neon_reg on operand 2
**Minimal input:** operands `[RegArrangement{v0,"8b"}, RegList({v0.16b}), Reg("x0")]`
**Expected:** Err
**Actual:** Ok(Word(...)) — `let (rm, _) = get_neon_reg(operands, 2)` accepts `Operand::Reg`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_bare_vm
**Serial reconfirmation:** same get_neon_reg path as encode_neon_tbl_neg_arity_kinds GPR dest; coverage-sweep kind=3
