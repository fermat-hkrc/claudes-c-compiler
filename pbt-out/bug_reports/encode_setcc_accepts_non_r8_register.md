# Bug: encode_setcc accepts non-r8 register operands via reg_num aliasing

**Law:** Intel SETcc encodes only r/m8. Register operands must be one of al/cl/dl/bl/ah/ch/dh/bh (or be rejected). llvm-mc `-triple=i686` rejects `sete %ax`, `sete %eax`, and non-GP names; the SUT must Err on those inputs rather than silently emitting the r8 encoding of the aliased 3-bit number.
**Impact:** Invalid assembly such as `sete %eax` or `sete %xmm0` is accepted and encoded as `sete %al` / `sete %al`, producing wrong machine code without diagnostics. Callers that accidentally pass a 16/32-bit name get silent mis-assembly.
**Function:** encode_setcc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1083
**Detected by:** Negative/error contract + llvm-mc differential gate (encode_setcc_neg_wrong_width, encode_setcc_neg_non_gp)
**Minimal input:** `sete %ax` → Ok([0x0f, 0x94, 0xc0]); also `sete %eax`, `sete %xmm0`
**Expected:** `Err(...)` (llvm-mc: `error: invalid operand for instruction`)
**Actual:** `Ok([0x0f, 0x94, 0xc0])` — same bytes as `sete %al`
**Severity:** high
**Root cause:** `gp_integer.rs:1083-1087` calls `reg_num(&reg.name)` with no `reg_size` gate. `registers.rs:4` maps ax/eax/xmm0/mm0/st onto the same 3-bit numbers as al, so invalid widths and non-GP names encode as if they were the corresponding r8.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1083`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.bytes.push(self.modrm(3, 0, num));
                Ok(())
            }
```
**Suggested fix:** Require an 8-bit GP name before encoding (use `reg_size` and reject xmm/mm/st/ymm prefixes).
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 1
                    || reg.name.starts_with("xmm")
                    || reg.name.starts_with("mm")
                    || reg.name.starts_with("st")
                    || reg.name.starts_with("ymm")
                {
                    return Err(format!("setcc requires r8 register, got {}", reg.name));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.bytes.push(self.modrm(3, 0, num));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_setcc_neg_wrong_width -- --test-threads=1 --exact
cargo test --lib test_encode_setcc_regression_rejects_ax -- --test-threads=1 --exact
cargo test --lib test_encode_setcc_regression_rejects_eax -- --test-threads=1 --exact
```
**Raw output:**
```text
Test failed: SUT accepted invalid-width `sete %ax` → [0f, 94, c0]; SETcc is r/m8 only (Intel SDM; llvm-mc rejects).
minimal failing input: r = "ax", m = "sete"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_setcc_pbt.rs (`test_encode_setcc_regression_rejects_ax`, `test_encode_setcc_regression_rejects_eax`)
