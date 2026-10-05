# Bug: encode_dmb ignores Operand::Imm and encodes SY
**Law:** For every CRm in 0..=15, encode_dmb([Imm(CRm)]) must equal llvm-mc("dmb #CRm") / GNU as `dmb #CRm` (ARM DMB with that CRm nibble), not DMB SY
**Impact:** Valid GNU-style `dmb #imm` (and bare `dmb imm`) assembles as `dmb sy` (CRm=15). A requested `#0`/`#4`/`#8` outer-shareable or reserved encoding becomes a full-system barrier, changing memory-ordering semantics of any assembly that uses the numeric form.
**Function:** encode_dmb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:6
**Detected by:** Differential — llvm-mc AArch64 assembler (LLVM 15.0.6)
**Minimal input:** encode_dmb(&[Operand::Imm(0)])  (assembly: `dmb #0`)
**Expected:** Word(0xd50330bf)  // CRm=0, matches llvm-mc and GNU as
**Actual:** Word(0xd5033fbf)  // CRm=15 (SY)
**Severity:** high
**Root cause:** system.rs:23 `_ => 0b1111` — Imm is not matched, so every immediate falls through to the SY default. Parser produces Imm for `#n` and bare integers (parser.rs:1991, 2020), and encode() passes operands through (mod.rs:964).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:23`
```rust
        _ => 0b1111,
```
**Suggested fix:** Treat Imm in 0..=15 as CRm; reject immediates outside that range.
```rust
        Some(Operand::Imm(n)) if (0..=15).contains(n) => *n as u32,
        Some(Operand::Imm(n)) => return Err(format!("dmb immediate out of range: {}", n)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dmb_regression_imm_crm0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dmb_pbt::encode_dmb_diff_imm' panicked at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:264:1:
Test failed: assertion failed: `(left == right)`
  left: `3573759935`,
 right: `3573756095`: SUT vs llvm-mc for dmb #0 at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:293.
minimal failing input: crm = 0
proptest: cc b237a050e3f0cc9e9ef0407f2594424cd2a7b9c9d331ee1871ff9d1b6d9d4c57
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
