# Bug: encode_cbz treats SP as XZR
**Law:** ARM CBZ/CBNZ Rt is Wt/Xt; register 31 is XZR/WZR, not SP/WSP. llvm-mc rejects `cbz sp, L` and `cbz wsp, L`. encode_cbz must Err when Rt is sp or wsp.
**Impact:** `cbz sp, L` is assembled as `cbz xzr, L` (encoding 0xb400001f with CondBr19). The object file writes a compare-and-branch on XZR instead of rejecting the stack pointer.
**Function:** encode_cbz
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("sp"), Symbol("L")], is_nz=false
**Expected:** Err (invalid operand; SP is not a valid CBZ register)
**Actual:** Ok(WordWithReloc { word: 0xb400001f, reloc: CondBr19, symbol: "L", addend: 0 }) — same encoding as `cbz xzr, L`. parse_reg_num maps sp to 31.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cbz_pbt::test_encode_cbz_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cbz -- --test-threads=1 reproduced the failure.
