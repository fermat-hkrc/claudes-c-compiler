# Bug: encode_push16 rejects segment-register pushw
**Law:** ∀ s ∈ {es,cs,ss,ds,fs,gs}. encode_push16([Reg(s)]) must equal llvm-mc `pushw %s` (66-prefixed classic Sreg PUSH opcodes).
**Impact:** `pushw %es` / `%fs` / etc. fail to assemble; 16-bit segment save sequences used in real-mode/V86 and legacy OS code cannot be encoded.
**Function:** encode_push16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:382
**Detected by:** Differential — llvm-mc i686 (encode_push16_diff_sreg)
**Minimal input:** `pushw %es` → Operand::Register("es")
**Expected:** `[0x66, 0x06]`
**Actual:** `Err("unsupported pushw operand")`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400` — Register (including Sreg) hits the catch-all Err. No Sreg table exists in encode_push16 (sibling encode_pop16 has one for POP; encode_push also lacks Sreg for 32-bit and has a related prior bug).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Handle segment registers with the classic PUSH Sreg opcodes plus 0x66:
```rust
            Operand::Register(reg) if is_segment_reg(&reg.name) => {
                self.bytes.push(0x66);
                match reg.name.as_str() {
                    "es" => { self.bytes.push(0x06); Ok(()) }
                    "cs" => { self.bytes.push(0x0E); Ok(()) }
                    "ss" => { self.bytes.push(0x16); Ok(()) }
                    "ds" => { self.bytes.push(0x1E); Ok(()) }
                    "fs" => { self.bytes.extend_from_slice(&[0x0F, 0xA0]); Ok(()) }
                    "gs" => { self.bytes.extend_from_slice(&[0x0F, 0xA8]); Ok(()) }
                    _ => Err(format!("cannot push {}", reg.name)),
                }
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push16_diff_sreg -- --test-threads=1
cargo test --lib test_encode_push16_regression_sreg_unsupported -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid Sreg form `pushw %es`: unsupported pushw operand; Intel PUSH Sreg with operand-size override must encode under pushw..
minimal failing input: sreg = "es"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push16_pbt.rs::test_encode_push16_regression_sreg_unsupported
