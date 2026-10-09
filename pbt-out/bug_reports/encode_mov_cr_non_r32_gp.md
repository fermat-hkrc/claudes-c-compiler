# Bug: encode_mov_cr accepts non-r32 GP operands
**Law:** MOV to/from a control register on IA-32 encodes only with a 32-bit general-purpose register (r32); 8-bit and 16-bit names, and `movw`/`movb` forms, must be rejected.
**Impact:** `movl %cr0, %ax` and `movl %al, %cr0` are assembled as if the operand were `%eax`, emitting `0F 20 C0` / `0F 22 C0`. Callers that accidentally use a narrow register alias get a silent wrong-width CR move identical to the r32 form; `movw %cr0, %ax` is likewise accepted. llvm-mc `-triple=i686` rejects these operands (`invalid operand for instruction`).
**Function:** encode_mov_cr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:250
**Detected by:** Negative/error contract (4) + differential disagreement with llvm-mc; also algebraic width gate
**Minimal input:** `movl %cr0, %ax` (also `movl %al, %cr0`, `movw %cr0, %ax`)
**Expected:** `Err` (MOV CR requires r32; llvm-mc rejects)
**Actual:** `Ok([0x0f, 0x20, 0xc0])` — same bytes as `movl %cr0, %eax`
**Severity:** medium
**Root cause:** `system.rs:257-258` / `263-264` resolve the GP via `reg_num`, which aliases `al`/`ax`/`eax` to the same 3-bit index, with no `reg_size == 4` gate; `encode_mov` also routes CR pairs into `encode_mov_cr` for any size mnemonic.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:255`
```rust
            (Operand::Register(cr), Operand::Register(gp)) if is_control_reg(&cr.name) => {
                let cr_num = control_reg_num(&cr.name).ok_or("bad control register")?;
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x20]);
                self.bytes.push(self.modrm(3, cr_num, gp_num));
                Ok(())
            }
```
**Suggested fix:** Reject non-r32 GP names before encoding (both arms); optionally reject when caller used movw/movb.
```rust
            (Operand::Register(cr), Operand::Register(gp)) if is_control_reg(&cr.name) => {
                let cr_num = control_reg_num(&cr.name).ok_or("bad control register")?;
                if reg_size(&gp.name) != 4 {
                    return Err("mov cr requires 32-bit register".to_string());
                }
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x20]);
                self.bytes.push(self.modrm(3, cr_num, gp_num));
                Ok(())
            }
            (Operand::Register(gp), Operand::Register(cr)) if is_control_reg(&cr.name) => {
                let cr_num = control_reg_num(&cr.name).ok_or("bad control register")?;
                if reg_size(&gp.name) != 4 {
                    return Err("mov cr requires 32-bit register".to_string());
                }
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x22]);
                self.bytes.push(self.modrm(3, cr_num, gp_num));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_cr_regression_rejects_ax -- --test-threads=1
cargo test --lib test_encode_mov_cr_regression_rejects_al -- --test-threads=1
cargo test --lib encode_mov_cr_neg_bad_operands -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid-width GP `movl %cr0, %ax` → [0f, 20, c0]; MOV CR requires r32 (Intel SDM; llvm-mc rejects).
minimal failing input: kind = 3, cr = "cr0", gp32 = "eax", r16 = "ax", r8 = "al", seg = "es", imm = 0

movl %cr0, %ax must Err (r32 only); got Ok(Ok([15, 32, 192]))
movl %al, %cr0 must Err (r32 only); got Ok(Ok([15, 34, 192]))
SUT accepted `movw %cr0, %ax` → [0f, 20, c0]; MOV CR is r32-only
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs (`test_encode_mov_cr_regression_rejects_ax`, `test_encode_mov_cr_regression_rejects_al`)
