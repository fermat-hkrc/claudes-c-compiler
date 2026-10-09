# Bug: encode_lmsw accepts 32-bit and 8-bit registers (r/m16 only)

**Law:** `encode_lmsw` must accept only a 16-bit GP register or memory operand (Intel SDM LMSW r/m16; doc comment "Accepts a 16-bit register or memory operand"; llvm-mc `-triple=i686` rejects `%eax`/`%al`).
**Impact:** Callers that pass a 32-bit or 8-bit register name get a silent encoding that is byte-identical to the corresponding 16-bit form (via `reg_num` aliasing: eax→0 same as ax). That hides width mistakes and produces encodings llvm-mc/gas reject, so assembled privileged code diverges from the assembler contract.
**Function:** encode_lmsw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:208
**Detected by:** Negative/error contract — encode_lmsw_neg_bad_operand (differential cross-check with llvm-mc reject)
**Minimal input:** `lmsw %eax` → SUT Ok(`[0f, 01, f0]`); llvm-mc rejects; same bytes as `lmsw %ax`
**Expected:** `Err(...)` (non-r16 register rejected)
**Actual:** `Ok([0x0f, 0x01, 0xf0])`
**Severity:** medium
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:208-212` — register arm calls `reg_num(&reg.name)` which aliases eax/ax/al to the same 3-bit encoding, with no `reg_size == 2` gate despite the doc claiming 16-bit registers only.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:208`
```rust
            Operand::Register(reg) => {
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.bytes.push(self.modrm(3, 6, rm));
                Ok(())
            }
```
**Suggested fix:** Reject non-16-bit register names before encoding (same pattern as the intended VERW/LMSW r/m16 gate).
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 2 {
                    return Err(format!("lmsw requires 16-bit register, got {}", reg.name));
                }
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.bytes.push(self.modrm(3, 6, rm));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lmsw_neg_bad_operand -- --test-threads=1
cargo test --lib test_encode_lmsw_regression_rejects_eax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid-width register `lmsw %eax` → [0f, 01, f0]; LMSW requires r/m16 (doc: 16-bit register; llvm-mc rejects).
minimal failing input: kind = 2, bad_reg = "eax", imm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs (test_encode_lmsw_regression_rejects_eax, test_encode_lmsw_regression_rejects_al)
