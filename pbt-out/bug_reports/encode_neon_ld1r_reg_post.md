# Bug: encode_neon_ld1r encodes register post-index as no-offset
**Law:** encode_neon_ld1r([RegList({Vt.T}), Mem{Xn}, Reg(Xm)]) must equal llvm-mc("ld1r {Vt.T}, [Xn], Xm") (L=1, Rm=Xm).
**Impact:** Valid `ld1r {v0.8b}, [x0], x0` is assembled as no-writeback `ld1r {v0.8b}, [x0]`. Post-index address arithmetic is dropped, so subsequent loads use the old base.
**Function:** encode_neon_ld1r
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:832
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_ld1r([RegList({v0.8b}), Mem{x0, 0}, Reg("x0")])
**Expected:** Word(0x0dc0c000) — llvm-mc `ld1r {v0.8b}, [x0], x0`
**Actual:** Word(0x0d40c000) — no-offset `ld1r {v0.8b}, [x0]`
**Severity:** high
**Root cause:** neon.rs:866 matches only operands[1] as Mem/MemPostIndex and never reads a trailing GPR. README.md:235 lists ld1r with post-index; ARM AdvSIMD register post-index is L=1 Rm=Xm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:866`
```rust
        Operand::Mem { base, offset: 0 } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            // LD1R: 0 Q 0 01101 0 1 0 00000 110 0 size Rn Rt (no post-index)
            let word = (q << 30) | (0b001101 << 24) | (1 << 22) | (0b110 << 13)
                | (size << 10) | (rn << 5) | rt;
            Ok(EncodeResult::Word(word))
        }
```
**Suggested fix:** If operands[2] is Xm, set L=1 and Rm=Xm.
```rust
        Operand::Mem { base, offset: 0 } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            if let Some(Operand::Reg(rm_name)) = operands.get(2) {
                let rm = parse_reg_num(rm_name).ok_or("invalid post-index reg")?;
                let word = (q << 30) | (0b001101 << 24) | (1 << 23) | (1 << 22)
                    | (rm << 16) | (0b110 << 13) | (size << 10) | (rn << 5) | rt;
                return Ok(EncodeResult::Word(word));
            }
            let word = (q << 30) | (0b001101 << 24) | (1 << 22) | (0b110 << 13)
                | (size << 10) | (rn << 5) | rt;
            Ok(EncodeResult::Word(word))
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_reg_post -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld1r_pbt::encode_neon_ld1r_diff_post_reg_llvm_mc' panicked at src/backend/arm/assembler/encoder/neon.rs:12939:5:
Test failed: assertion failed: `(left == right)`
  left: `222347264`,
 right: `230735872`: mismatch for ld1r {v0.8b}, [x0], x0 at src/backend/arm/assembler/encoder/neon.rs:13135.
minimal failing input: t = "8b", rt = 0, rn = 0, rm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
