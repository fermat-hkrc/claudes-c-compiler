# Bug: encode_mov_seg accepts 8-bit GP registers via reg_num aliasing
**Law:** MOV to/from a segment register is defined for r/m16 (and r32 on IA-32); 8-bit GP registers are invalid and must be rejected, matching llvm-mc and Intel SDM.
**Impact:** Forms like `movl %al, %ds` or `movl %es, %al` silently encode as if the low 3 bits of the r8 name were an r32 (`al`→same ModRM.rm as `eax`), producing wrong machine code for an illegal operand.
**Function:** encode_mov_seg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:293
**Detected by:** Negative/error contract — encode_mov_seg_neg_r8
**Minimal input:** `movl %es, %al` (and `movl %al, %ds`)
**Expected:** `Err(...)`
**Actual:** `Ok([0x8c, 0xc0])` / `Ok([0x8e, 0xd8])`
**Severity:** medium
**Root cause:** Both register arms call `reg_num(&gp.name)` (registers.rs:4-16), which maps `al`/`ah`/… onto the same 3-bit codes as `eax`/…. There is no `reg_size` gate before encoding.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:293`
```rust
            (Operand::Register(src), Operand::Register(dst)) if is_segment_reg(&src.name) => {
                let sr = seg_num(&src.name).ok_or("bad segment register")?;
                let gp = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(0x8C);
                self.bytes.push(self.modrm(3, sr, gp));
                Ok(())
            }
            // mov %reg32, %sreg
            (Operand::Register(src), Operand::Register(dst)) if is_segment_reg(&dst.name) => {
                let gp = reg_num(&src.name).ok_or("bad register")?;
                let sr = seg_num(&dst.name).ok_or("bad segment register")?;
                self.bytes.push(0x8E);
                self.bytes.push(self.modrm(3, sr, gp));
                Ok(())
            }
```
**Suggested fix:** Reject GP operands with `reg_size == 1` on both arms (keep r16 and r32).
```rust
                let gp_name = if is_segment_reg(&src.name) { &dst.name } else { &src.name };
                if reg_size(gp_name) == 1 {
                    return Err(format!("MOV Sreg does not accept 8-bit register {gp_name}"));
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_seg_regression_rejects_al -- --test-threads=1
```
**Raw output:**
```text
movl %al, %ds must Err (not r8); got Ok(Ok([142, 216]))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs (test_encode_mov_seg_regression_rejects_al)
