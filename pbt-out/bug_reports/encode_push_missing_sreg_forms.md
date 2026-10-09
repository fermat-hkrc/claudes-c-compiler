# Bug: encode_push rejects valid segment-register PUSH forms
**Law:** Intel SDM PUSH Sreg is a valid instruction: ES=0x06, CS=0x0E, SS=0x16, DS=0x1E, FS=0x0F 0xA0, GS=0x0F 0xA8. Sibling `encode_pop` already implements the POP Sreg table.
**Impact:** `push %es` / `push %fs` / etc. fail to assemble (`bad register`), so legitimate AT&T that llvm-mc and GAS accept cannot be produced by this encoder.
**Function:** encode_push
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351
**Detected by:** Differential — llvm-mc i686 (5)
**Minimal input:** `pushl %es`
**Expected:** `Ok([0x06])`
**Actual:** `Err("bad register")`
**Severity:** medium
**Root cause:** Register arm only calls `reg_num`, which has no Sreg entries (`registers.rs:4-15`). `is_segment_reg` exists and is used by sibling `encode_pop` (gp_integer.rs:408-416) but not by `encode_push`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Mirror `encode_pop`'s Sreg table with the PUSH opcodes.
```rust
            Operand::Register(reg) => {
                if is_segment_reg(&reg.name) {
                    return match reg.name.as_str() {
                        "es" => { self.bytes.push(0x06); Ok(()) }
                        "cs" => { self.bytes.push(0x0E); Ok(()) }
                        "ss" => { self.bytes.push(0x16); Ok(()) }
                        "ds" => { self.bytes.push(0x1E); Ok(()) }
                        "fs" => { self.bytes.extend_from_slice(&[0x0F, 0xA0]); Ok(()) }
                        "gs" => { self.bytes.extend_from_slice(&[0x0F, 0xA8]); Ok(()) }
                        _ => Err(format!("cannot push {}", reg.name)),
                    };
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push_regression_sreg_es -- --test-threads=1 --exact
```
**Raw output:**
```text
regression: encode_push must accept Sreg push %es, got Err(bad register)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push_pbt.rs (encode_push_regression_sreg_es)
