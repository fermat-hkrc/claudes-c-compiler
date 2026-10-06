# Bug: encode_tst encodes SP/WSP as XZR/WZR
**Law:** TST Rn and Rm must be GPRs (Xn/Wn or XZR/WZR), not SP/WSP.
**Impact:** `tst sp, x0` is silently encoded as `tst xzr, x0` (0xea0003ff). A stack-pointer operand is turned into ZR, so invalid assembly becomes a different, well-formed instruction. llvm-mc and gas reject SP/WSP.
**Function:** encode_tst
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:37
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tst([Reg("sp"), Reg("x0")])
**Expected:** Err
**Actual:** Ok(Word(0xea0003ff)) — SP number 31 is the same encoding as XZR
**Severity:** medium
**Root cause:** compare_branch.rs:41-49 only inspects is_32bit_reg on the first operand (false for "sp"), prepends xzr, then encode_logical/parse_reg_num maps both "sp" and "xzr" to 31, so ANDS XZR, SP, X0 equals ANDS XZR, XZR, X0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/mod.rs:266`
```rust
        "sp" | "wsp" => Some(31),
        "xzr" | "wzr" => Some(31),
```
**Suggested fix:** Reject SP/WSP in encode_tst before the ANDS alias.
```rust
    fn is_sp(name: &str) -> bool {
        matches!(name.to_ascii_lowercase().as_str(), "sp" | "wsp")
    }
    if operands.iter().any(|o| matches!(o, Operand::Reg(r) if is_sp(r))) {
        return Err("tst: SP/WSP is not a valid GPR".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tst_regression_sp_rn -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tst_pbt::encode_tst_neg_sp stdout ----
Test failed: tst SP/WSP kind=0 must Err (llvm-mc: invalid operand) at src/backend/arm/assembler/encoder/encode_tst_pbt.rs:647.
minimal failing input: which = 0, n = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tst_pbt.rs
