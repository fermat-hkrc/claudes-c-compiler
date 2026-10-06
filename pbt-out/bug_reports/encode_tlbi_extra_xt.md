# Bug: encode_tlbi accepts a register on no-Xt TLBI ops
**Law:** No-Xt TLBI operations (VMALLE1, VMALLE1IS, ALLE1, ALLE1IS, ALLE2IS, VMALLS12E1, VMALLS12E1IS) must not take a register operand
**Impact:** `tlbi vmalle1is, x0` is assembled as SYS with Rt=x0 instead of being rejected. llvm-mc/gas report "specified tlbi op does not use a register". The emitted word is a different (and for these ops, reserved/invalid) encoding
**Function:** encode_tlbi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:477
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tlbi([], "vmalle1is, x0")
**Expected:** Err (llvm-mc: "specified tlbi op does not use a register")
**Actual:** Ok(Word(0xd5088300)) — VMALLE1IS with Rt=x0 instead of XZR
**Severity:** medium
**Root cause:** system.rs:480-485 parse an optional Rt for every op, then system.rs:537 patches bits[4:0] even when the matched op's architectural Rt must be 0b11111
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:537`
```rust
    let word = (base & !0x1F) | rt;
```
**Suggested fix:** Reject a second operand on no-Xt ops; do not patch Rt away from the architectural XZR value
```rust
    if !needs_xt(op_name) && parts.len() > 1 {
        return Err(format!("tlbi: {} does not use a register", op_name));
    }
    let word = (base & !0x1F) | rt;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_extra_xt -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_tlbi_pbt::test_encode_tlbi_regression_extra_xt' panicked at src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs:377:39:
TLBI VMALLE1IS takes no Xt (llvm-mc: specified tlbi op does not use a register): Word(3574104832)
Test failed: no-Xt TLBI with register must Err (llvm-mc rejects tlbi vmalle1is, x0); SUT raw "vmalle1is, x0": Word(3574104832).
minimal failing input: op = "vmalle1is", xt = "x0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
