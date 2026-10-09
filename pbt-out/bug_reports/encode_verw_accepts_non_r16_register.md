# Bug: encode_verw accepts 32-bit and 8-bit registers (VERW is r/m16 only)
**Law:** VERW takes an r/m16 selector operand. Register form must accept only 16-bit GP names (ax/bx/cx/dx/sp/bp/si/di); 32-bit (eax…) and 8-bit (al/ah…) registers must be rejected.
**Impact:** Invalid assembly such as `verw %eax` is silently encoded as if it were `verw %ax` (same ModR/M via `reg_num`), producing machine code the programmer did not request and that llvm-mc/gas reject.
**Function:** encode_verw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:133
**Detected by:** Negative/error contract + differential vs llvm-mc (rejects %eax/%al)
**Minimal input:** `verw %eax` → Ok([0x0f, 0x00, 0xe8]) (identical to `verw %ax`)
**Expected:** `Err(...)` (invalid operand width)
**Actual:** `Ok([0x0f, 0x00, 0xe8])`
**Severity:** medium
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:133-138` — register arm uses `reg_num` only, which aliases al/ax/eax to the same 3-bit code; no width check against r/m16.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:133`
```rust
            Operand::Register(reg) => {
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x00]);
                self.bytes.push(self.modrm(3, 5, rm));
                Ok(())
            }
```
**Suggested fix:** Require 16-bit GP names (or `reg_size(&reg.name) == 2`) before encoding.
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 2 || is_segment_reg(&reg.name) {
                    return Err(format!("verw requires r/m16 register, got {}", reg.name));
                }
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x00]);
                self.bytes.push(self.modrm(3, 5, rm));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_verw_regression_rejects_eax -- --test-threads=1
```
**Raw output:**
```text
verw %eax must Err (r/m16 only); got Ok(Ok([15, 0, 232]))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_verw_pbt.rs (test_encode_verw_regression_rejects_eax)
