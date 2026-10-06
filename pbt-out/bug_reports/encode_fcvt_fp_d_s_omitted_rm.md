# Bug: encode_fcvt_fp encodes omitted rm as DYN on FCVT.D.S; llvm-mc encodes RNE
**Law:** A 2-operand FCVT.D.S must encode the same 32-bit word as llvm-mc for the same assembly. llvm-mc encodes omitted rm as RNE (000) on this mnemonic because float32→double is exact.
**Impact:** `fcvt.d.s f0, f0` assembled by this encoder has rm=DYN (0x42007053) while llvm-mc produces rm=RNE (0x42000053). Object files disagree; the conversion is exact so the numerical result is unchanged. llvm-mc also rejects an explicit rm operand on FCVT.D.S, which this encoder still accepts.
**Function:** encode_fcvt_fp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:146
**Detected by:** Differential
**Minimal input:** encode_fcvt_fp([Reg("f0"), Reg("f0")], 0b0100001, 0) compared with llvm-mc("fcvt.d.s f0, f0")
**Expected:** 0x42000053 (llvm-mc, rm=RNE)
**Actual:** 0x42007053 (SUT, rm=DYN)
**Severity:** low
**Root cause:** float.rs:156 always uses 0b111 (DYN) when operands.len() <= 2, including FCVT.D.S where llvm-mc hardwires RNE because single-to-double is exact.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:156`
```rust
        0b111
```
**Suggested fix:** For FCVT.D.S (funct7=0100001 and rs2=0), encode omitted rm as RNE to match llvm-mc, or accept that object files will differ. Matching the differential reference:
```rust
        if funct7 == 0b0100001 && rs2 == 0 {
            0b000
        } else {
            0b111
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_fp -- --test-threads=1
cargo test --lib test_encode_fcvt_fp_regression_fcvt_d_s_default_rm -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `1107325011`,
 right: `1107296339`: SUT 42007053 != llvm-mc 42000053 for fcvt.d.s f0, f0 at src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs:374.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.d.s",
    33,
    0,
), rd = "f0", rs1 = "f0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
