# Bug: encode_fcvt_from_int encodes omitted rm as DYN on FCVT.D.W/WU; llvm-mc encodes RNE
**Law:** A 2-operand FCVT.D.W/WU must encode the same 32-bit word as llvm-mc for the same assembly. llvm-mc encodes omitted rm as RNE (000) on these mnemonics.
**Impact:** `fcvt.d.w f0, x0` assembled by this encoder has rm=DYN (0xd2007053) while llvm-mc produces rm=RNE (0xd2000053). Object files disagree; the conversion is exact so the numerical result is unchanged.
**Function:** encode_fcvt_from_int
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:131
**Detected by:** Differential
**Minimal input:** encode_fcvt_from_int([Reg("f0"), Reg("x0")], 0b1101001, 0) compared with llvm-mc("fcvt.d.w f0, x0")
**Expected:** 0xd2000053 (llvm-mc, rm=RNE)
**Actual:** 0xd2007053 (SUT, rm=DYN)
**Severity:** low
**Root cause:** float.rs:141 always uses 0b111 (DYN) when operands.len() <= 2, including FCVT.D.W/WU where llvm-mc hardwires RNE because int32→double is exact.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:141`
```rust
        0b111
```
**Suggested fix:** For FCVT.D.W/WU (funct7=1101001 and rs2 in {0,1}), encode omitted rm as RNE to match llvm-mc, or accept that object files will differ. Matching the differential reference:
```rust
        if funct7 == 0b1101001 && (rs2 == 0 || rs2 == 1) {
            0b000
        } else {
            0b111
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_from_int -- --test-threads=1
cargo test --lib test_encode_fcvt_from_int_regression_dw_omitted_rm -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `3523244115`,
 right: `3523215443`: SUT d2007053 != llvm-mc d2000053 for fcvt.d.w f0, x0
minimal failing input: (mn, f7, rs2) = (
    "fcvt.d.w",
    105,
    0,
), rd = "f0", rs1 = "x0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
