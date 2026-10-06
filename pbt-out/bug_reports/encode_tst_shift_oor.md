# Bug: encode_tst wraps out-of-range shift amounts
**Law:** TST shifted-register form allows shift amount 0..31 on W registers and 0..63 on X registers; larger amounts must be rejected.
**Impact:** `tst w0, w0, lsl #32` is encoded with imm6=32 (0x6a00801f) instead of being rejected. llvm-mc and gas reject the amount. The wrapped encoding is UNALLOCATED in the 32-bit shifted-register space (imm6 bit 5 must be 0).
**Function:** encode_tst
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:37
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tst([Reg("w0"), Reg("w0"), Shift { kind: "lsl", amount: 32 }])
**Expected:** Err
**Actual:** Ok(Word(0x6a00801f)) — amount is masked with `& 0x3F` and packed into imm6
**Severity:** medium
**Root cause:** encode_logical packs `shift_amount & 0x3F` with no 32-bit max-31 / 64-bit max-63 check (data_processing.rs:496); encode_tst forwards the Shift operand unchanged.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:496`
```rust
            | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
```
**Suggested fix:** Reject out-of-range shift amounts (in encode_tst or encode_logical).
```rust
    let max = if is_64 { 63u32 } else { 31u32 };
    if shift_amount > max {
        return Err(format!("shift amount {shift_amount} out of range 0..{max}"));
    }
    let word = ((sf << 31) | (opc << 29) | (0b01010 << 24) | (shift_type << 22))
        | (rm << 16) | (shift_amount << 10) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tst_regression_shift_oor -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tst_pbt::encode_tst_neg_shift_oor stdout ----
Test failed: tst shift lsl #32 on W must Err (llvm-mc range 0..31) at src/backend/arm/assembler/encoder/encode_tst_pbt.rs:747.
minimal failing input: rn = 0, rm = 0, is_64 = false, sk = 0, amt = 32
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tst_pbt.rs
