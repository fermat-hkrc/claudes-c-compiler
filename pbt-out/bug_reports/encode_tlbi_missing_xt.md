# Bug: encode_tlbi defaults a missing Xt to XZR instead of rejecting Xt-required ops
**Law:** Xt-required TLBI operations (VAE*/VALE*/VAAE*/VAALE*/ASIDE*/IPAS2*/R*) must take a 64-bit GPR; omitting Xt is invalid
**Impact:** The assembler silently emits a well-formed SYS word with Rt=XZR for `tlbi vale1is` (and every other Xt-required op). Callers that drop the register get a different invalidate than the text they wrote, and GNU gas / llvm-mc reject the same input
**Function:** encode_tlbi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:477
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tlbi([], "vale1is")
**Expected:** Err (llvm-mc: "specified tlbi op requires a register")
**Actual:** Ok(Word(0xd50883bf)) — VALE1IS with Rt=XZR
**Severity:** medium
**Root cause:** system.rs:483-485 — when no comma is present the encoder sets Rt=31 rather than checking that the matched op requires Xt
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:483`
```rust
    } else {
        31 // xzr
    };
```
**Suggested fix:** For Xt-required ops, return Err when the register operand is absent; keep Rt=31 only for no-Xt ops (VMALLE*/ALLE*/VMALLS12E1*)
```rust
    } else if needs_xt(op_name) {
        return Err(format!("tlbi: {} requires a register", op_name));
    } else {
        31 // xzr
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_missing_xt -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_tlbi_pbt::test_encode_tlbi_regression_missing_xt' panicked at src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs:371:32:
TLBI VAE1IS requires Xt (llvm-mc: specified tlbi op requires a register): Word(3574104895)
Test failed: Xt-required TLBI without register must Err (llvm-mc rejects tlbi vale1is); SUT raw "vale1is": Word(3574105023).
minimal failing input: raw = "vale1is"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
