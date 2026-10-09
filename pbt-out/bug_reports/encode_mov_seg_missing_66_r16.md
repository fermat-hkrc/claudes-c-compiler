# Bug: encode_mov_seg omits 0x66 for movw Sreg→r16
**Law:** When the general-purpose destination of MOV from a segment register is a 16-bit register (`movw %sreg, %r16`), the encoding must include the operand-size override prefix 0x66 before 0x8C, matching llvm-mc and Intel SDM (r/m16 form).
**Impact:** `movw %ds, %ax` assembles as the 32-bit form `[8c,d8]` instead of `[66,8c,d8]`, so a 16-bit intent silently becomes a 32-bit store into EAX's encoding path / wrong machine code for the declared width.
**Function:** encode_mov_seg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:293
**Detected by:** Differential — llvm-mc i686 (encode_mov_seg_diff_sreg_to_r16)
**Minimal input:** `movw %es, %ax`
**Expected:** `[0x66, 0x8c, 0xc0]`
**Actual:** `[0x8c, 0xc0]`
**Severity:** high
**Root cause:** system.rs:293-298 (Sreg→GP arm) always emits bare `0x8C` + ModRM. `encode_mov` routes segment forms to `encode_mov_seg` without passing the mnemonic size (gp_integer.rs:21-23), so `movw` loses the need for 0x66. Writing TO a segment (`movw %ax, %ds`) correctly needs no 0x66; only the Sreg→r16 direction does.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:293`
```rust
            // mov %sreg, %reg32
            (Operand::Register(src), Operand::Register(dst)) if is_segment_reg(&src.name) => {
                let sr = seg_num(&src.name).ok_or("bad segment register")?;
                let gp = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(0x8C);
                self.bytes.push(self.modrm(3, sr, gp));
                Ok(())
            }
```
**Suggested fix:** Emit 0x66 when the GP destination is 16-bit (and reject 8-bit).
```rust
            (Operand::Register(src), Operand::Register(dst)) if is_segment_reg(&src.name) => {
                let sr = seg_num(&src.name).ok_or("bad segment register")?;
                let gp = reg_num(&dst.name).ok_or("bad register")?;
                let sz = reg_size(&dst.name);
                if sz == 1 {
                    return Err(format!("MOV Sreg does not accept 8-bit register {}", dst.name));
                }
                if sz == 2 {
                    self.bytes.push(0x66);
                }
                self.bytes.push(0x8C);
                self.bytes.push(self.modrm(3, sr, gp));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_seg_regression_movw_ds_ax_66 -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: movw %ds, %ax must be [66, 8c, d8]
  left: [140, 216]
 right: [102, 140, 216]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs (test_encode_mov_seg_regression_movw_ds_ax_66)
