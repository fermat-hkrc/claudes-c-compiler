# Bug: encode_pop16 omits 0x66 on segment-register popw
**Law:** ∀ s ∈ {es,ss,ds,fs,gs}. encode_pop16([s]) must equal llvm-mc `popw %s`, which prefixes the classic Sreg POP opcode with operand-size override 0x66 in 32-bit code.
**Impact:** Assembler emits wrong machine code for `popw %es`/`%ss`/`%ds`/`%fs`/`%gs`. Callers that assemble 16-bit-sized segment pops get the 32-bit default-size encoding instead, breaking binary compatibility with gas/llvm-mc and any consumer that expects the 0x66 prefix.
**Function:** encode_pop16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:325
**Detected by:** Differential (llvm-mc i686) + Algebraic metamorphic (popw Sreg = 0x66 ‖ popl Sreg)
**Minimal input:** `popw %es` → SUT `[0x07]`, llvm-mc `[0x66, 0x07]`
**Expected:** `[0x66, 0x07]` (and `[0x66, 0x0F, 0xA1]` for fs, `[0x66, 0x0F, 0xA9]` for gs)
**Actual:** `[0x07]` (fs: `[0x0F, 0xA1]`, gs: `[0x0F, 0xA9]`)
**Severity:** high
**Root cause:** system.rs:333-336 hard-codes Sreg POP opcodes without the 0x66 prefix that the `popw` mnemonic requires; the inline comment incorrectly asserts segment pops never use 0x66 (true for `popl`, false for `popw`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:333`
```rust
                if is_segment_reg(&reg.name) {
                    // Segment register pops don't use 0x66 prefix
                    match reg.name.as_str() {
                        "es" => { self.bytes.push(0x07); Ok(()) }
                        "ss" => { self.bytes.push(0x17); Ok(()) }
                        "ds" => { self.bytes.push(0x1F); Ok(()) }
                        "fs" => { self.bytes.extend_from_slice(&[0x0F, 0xA1]); Ok(()) }
                        "gs" => { self.bytes.extend_from_slice(&[0x0F, 0xA9]); Ok(()) }
                        _ => Err(format!("cannot pop to {}", reg.name)),
                    }
```
**Suggested fix:** Emit 0x66 before the Sreg opcode for `popw` (leave `encode_pop`/`popl` unchanged):
```rust
                if is_segment_reg(&reg.name) {
                    self.bytes.push(0x66); // popw requires operand-size override
                    match reg.name.as_str() {
                        "es" => { self.bytes.push(0x07); Ok(()) }
                        "ss" => { self.bytes.push(0x17); Ok(()) }
                        "ds" => { self.bytes.push(0x1F); Ok(()) }
                        "fs" => { self.bytes.extend_from_slice(&[0x0F, 0xA1]); Ok(()) }
                        "gs" => { self.bytes.extend_from_slice(&[0x0F, 0xA9]); Ok(()) }
                        _ => Err(format!("cannot pop to {}", reg.name)),
                    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop16_diff_sreg -- --test-threads=1
cargo test --lib test_encode_pop16_regression_sreg_missing_66 -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[7]`,
 right: `[102, 7]`: Sreg popw must match llvm-mc (incl. 0x66) for popw %es
minimal failing input: sreg = "es"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_pop16_pbt.rs::test_encode_pop16_regression_sreg_missing_66
