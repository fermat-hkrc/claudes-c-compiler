# Bug: encode_dsb ignores Operand::Imm and encodes SY
**Law:** For every CRm in 0..=15, encode_dsb([Imm(CRm)]) must equal llvm-mc("dsb #CRm") / GNU as `dsb #CRm` (ARM DSB with that CRm nibble), not DSB SY
**Impact:** Valid GNU-style `dsb #imm` (and bare `dsb imm`) assembles as `dsb sy` (CRm=15). A requested `#0`/`#4`/`#8` outer-shareable or reserved encoding becomes a full-system barrier, changing memory-ordering semantics of any assembly that uses the numeric form.
**Function:** encode_dsb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:30
**Detected by:** Differential — llvm-mc AArch64 assembler (LLVM 15.0.6)
**Minimal input:** encode_dsb(&[Operand::Imm(0)])  (assembly: `dsb #0`)
**Expected:** Word(0xd503309f)  // CRm=0, matches llvm-mc and GNU as
**Actual:** Word(0xd5033f9f)  // CRm=15 (SY)
**Severity:** high
**Root cause:** system.rs:47 `_ => 0b1111` — Imm is not matched, so every immediate falls through to the SY default. Parser produces Imm for `#n` and bare integers (parser.rs:1991, 2020), and encode() passes operands through (mod.rs:967).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:47`
```rust
        _ => 0b1111,
```
**Suggested fix:** Treat Imm in 0..=15 as CRm; reject immediates outside that range.
```rust
        Some(Operand::Imm(n)) if (0..=15).contains(n) => *n as u32,
        Some(Operand::Imm(n)) => return Err(format!("dsb immediate out of range: {}", n)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_imm_crm0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dsb_pbt::encode_dsb_diff_imm' panicked at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:264:1:
Test failed: assertion failed: `(left == right)`
  left: `3573759903`,
 right: `3573756063`: SUT vs llvm-mc for dsb #0 at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:293.
minimal failing input: crm = 0
proptest: cc c5614437345ff82a5cc5a5d8c06e99a694c826b52825b26e31c45e7e2f0f733d
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
