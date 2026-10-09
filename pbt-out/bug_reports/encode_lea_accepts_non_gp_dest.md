# Bug: encode_lea accepts non-GP / wrong-width destinations via reg_num aliasing

**Law:** LEA destination must be a general-purpose r16 or r32 register (Intel SDM LEA r16,m / r32,m). Under the `leal` mnemonic the destination must be 32-bit GP; xmm/mm/st/ymm and r8 names must be rejected (llvm-mc rejects them).
**Impact:** Invalid assembly such as `leal (%eax), %xmm0` or `leal (%eax), %al` is silently encoded as if the destination were the aliased GP number (xmm0→0 → same bytes as `%eax`), producing wrong code without an assembler error.
**Function:** encode_lea
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:332
**Detected by:** Negative/error contract — encode_lea_neg_shape_and_dest (mode=4 non-GP)
**Minimal input:** `leal (%eax), %xmm0` (Memory base eax, Register xmm0)
**Expected:** `Err(...)` (llvm-mc rejects)
**Actual:** `Ok([0x8d, 0x00])` — same encoding as `leal (%eax), %eax`
**Severity:** medium
**Root cause:** `gp_integer.rs:338` uses `reg_num(&dst.name)` only. `registers.rs::reg_num` maps xmm/mm/st/ymm and r8 names onto 0–7, so non-GP and wrong-width destinations are accepted. `_size` is ignored and there is no `reg_size` / GP-class gate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:338`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.push(0x8D);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Require a GP destination of the mnemonic width (and emit 0x66 when size==2 / leaw is wired).
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let sz = reg_size(&dst.name);
                if is_xmm(&dst.name) || is_mm(&dst.name) || dst.name.starts_with("st")
                    || is_segment_reg(&dst.name) || sz == 1
                {
                    return Err(format!("lea bad dst register: {}", dst.name));
                }
                if size == 2 && sz != 2 {
                    return Err(format!("leaw requires r16 dest, got {}", dst.name));
                }
                if size != 2 && sz != 4 {
                    return Err(format!("leal requires r32 dest, got {}", dst.name));
                }
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.emit_segment_prefix(mem);
                if size == 2 {
                    self.bytes.push(0x66);
                }
                self.bytes.push(0x8D);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lea_regression_non_gp_xmm0_dest -- --test-threads=1
cargo test --lib encode_lea_neg_shape_and_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid LEA `leal (%eax), %xmm0` → [8d, 00]; LEA requires memory source and r16/r32 GP dest (Intel SDM). encode_lea uses reg_num which aliases r8/xmm/mm/st and ignores size..
minimal failing input: mode = 4, ri = 0, ni = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lea_pbt.rs (encode_lea_regression_non_gp_xmm0_dest)
