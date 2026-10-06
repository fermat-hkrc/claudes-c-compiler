# Bug: encode_mrs encodes write-only OSLAR_EL1
**Law:** If llvm-mc / GNU as reject `mrs Xt, oslar_el1` (OSLAR_EL1 is write-only), then encode_mrs([Reg(Xt), Symbol("oslar_el1")]) must return Err
**Impact:** `mrs x0, oslar_el1` assembles to a well-formed MRS word instead of being rejected. The resulting object executes a read of a write-only OS-lock register, which the architecture does not define as an MRS and which gas/llvm-mc refuse to assemble.
**Function:** encode_mrs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:54
**Detected by:** Differential — llvm-mc AArch64 assembler (named sysreg table)
**Minimal input:** encode_mrs(&[Operand::Reg("x0".into()), Operand::Symbol("oslar_el1".into())])  (assembly: `mrs x0, oslar_el1`)
**Expected:** Err (llvm-mc: "expected readable system register")
**Actual:** Ok(Word(0xd5301080))
**Severity:** medium
**Root cause:** system.rs:131 `"oslar_el1" => 0x8084` is in the MRS named table. OSLAR_EL1 is a write-only register (MSR only); ARM/llvm-mc reject it as an MRS source.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:131`
```rust
        "oslar_el1" => 0x8084,
```
**Suggested fix:** Drop `oslar_el1` from the MRS table so it falls through to `parse_generic_sysreg` and returns Err, or reject write-only names explicitly.
```rust
        // oslar_el1 is write-only (MSR); do not match it here
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_oslar_el1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_mrs_pbt::encode_mrs_diff_named' panicked at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:419:1:
Test failed: SUT encoded mrs x0, oslar_el1 as d5301080, llvm-mc rejected: llvm-mc error: <stdin>:1:9: error: expected readable system register
mrs x0, oslar_el1
        ^
 at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:443.
minimal failing input: name = "oslar_el1", xt = "x0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
