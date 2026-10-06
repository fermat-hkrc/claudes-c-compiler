# Bug: encode_msr encodes read-only OSLSR_EL1
**Law:** If llvm-mc / GNU as reject `msr oslsr_el1, Xt` (OSLSR_EL1 is read-only), then encode_msr([Symbol("oslsr_el1"), Reg(Xt)]) must return Err
**Impact:** `msr oslsr_el1, x0` assembles to a well-formed MSR word instead of being rejected. The resulting object executes a write of a read-only OS-lock status register, which the architecture does not define as an MSR and which gas/llvm-mc refuse to assemble.
**Function:** encode_msr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:262
**Detected by:** Differential — llvm-mc AArch64 assembler (named sysreg table)
**Minimal input:** encode_msr(&[Operand::Symbol("oslsr_el1".into()), Operand::Reg("x0".into())])  (assembly: `msr oslsr_el1, x0`)
**Expected:** Err (llvm-mc: "expected writable system register or pstate")
**Actual:** Ok(Word(0xd5101180))
**Severity:** medium
**Root cause:** system.rs:311 `"oslsr_el1" => 0x808c` is in the MSR named table. OSLSR_EL1 is a read-only register (MRS only); ARM/llvm-mc reject it as an MSR destination.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:311`
```rust
        "oslsr_el1" => 0x808c,
```
**Suggested fix:** Drop `oslsr_el1` from the MSR table so it falls through to `parse_generic_sysreg` and returns Err, or reject read-only names explicitly.
```rust
        // oslsr_el1 is read-only (MRS); do not match it here
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_oslsr_el1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_msr_pbt::test_encode_msr_regression_oslsr_el1' panicked at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:766:5:
msr oslsr_el1, x0 must Err (OSLSR_EL1 is read-only)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_msr_pbt.rs
