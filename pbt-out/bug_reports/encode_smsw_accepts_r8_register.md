# Bug: encode_smsw accepts 8-bit registers (r/m16 or r32/m16 only)

**Law:** `encode_smsw` must accept only a 16-bit or 32-bit GP register or memory operand (Intel SDM SMSW r/m16 and r32/m16; llvm-mc `-triple=i686` accepts `%ax`/`%eax` and rejects `%al`/`%ah`).
**Impact:** Callers that pass an 8-bit register name get a silent encoding that is byte-identical to the corresponding 32-bit form (via `reg_num` aliasing: al→0 same as eax, without the 0x66 prefix that ax would get). That hides width mistakes and produces encodings llvm-mc/gas reject, so assembled privileged code diverges from the assembler contract.
**Function:** encode_smsw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:230
**Detected by:** Negative/error contract — encode_smsw_neg_bad_operand (differential cross-check with llvm-mc reject)
**Minimal input:** `smsw %al` → SUT Ok(`[0f, 01, e0]`); llvm-mc rejects; same bytes as `smsw %eax`
**Expected:** `Err(...)` (r8 register rejected)
**Actual:** `Ok([0x0f, 0x01, 0xe0])`
**Severity:** medium
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:230-239` — register arm calls `reg_num(&reg.name)` which aliases al/ax/eax to the same 3-bit encoding, and only special-cases the eight r16 names for the 0x66 prefix; there is no gate rejecting `reg_size == 1`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:230`
```rust
            Operand::Register(reg) => {
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                // 16-bit register form needs operand size prefix
                let is_16 = matches!(reg.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
                if is_16 {
                    self.bytes.push(0x66);
                }
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.bytes.push(self.modrm(3, 4, rm));
                Ok(())
            }
```
**Suggested fix:** Reject 8-bit (and non-GP) register names before encoding; keep r16 (with 0x66) and r32 (without) as llvm-mc/Intel allow.
```rust
            Operand::Register(reg) => {
                let sz = reg_size(&reg.name);
                if sz != 2 && sz != 4 {
                    return Err(format!("smsw requires 16- or 32-bit register, got {}", reg.name));
                }
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                if sz == 2 {
                    self.bytes.push(0x66);
                }
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.bytes.push(self.modrm(3, 4, rm));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_smsw_neg_bad_operand -- --test-threads=1
cargo test --lib test_encode_smsw_regression_rejects_al -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid-width register `smsw %al` → [0f, 01, e0]; SMSW requires r/m16 or r32/m16 (llvm-mc rejects r8).
minimal failing input: kind = 2, bad_reg = "al", imm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_smsw_pbt.rs (test_encode_smsw_regression_rejects_al, test_encode_smsw_regression_rejects_ah)
