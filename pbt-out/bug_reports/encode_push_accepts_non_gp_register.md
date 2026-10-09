# Bug: encode_push accepts non-GP / wrong-width registers via reg_num alias
**Law:** PUSH register form is only valid for general-purpose r32 (and r16 with 0x66); XMM/MM/r8 are not legal PUSH operands and must be rejected.
**Impact:** `pushl %xmm0` / `pushl %al` assemble to `0x50` (same as `pushl %eax`), producing wrong machine code with no diagnostic — silent mis-assembly.
**Function:** encode_push
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351
**Detected by:** Negative/Error Contract (3); Differential — llvm-mc rejects non-GP
**Minimal input:** `pushl %xmm0` → SUT `Ok([0x50])`; `pushl %al` → SUT `Ok([0x50])`
**Expected:** `Err(...)` (llvm-mc: "invalid operand for instruction")
**Actual:** `Ok([0x50])` (encodes as PUSH EAX)
**Severity:** high
**Root cause:** `gp_integer.rs:351-354` uses `reg_num` only. `registers.rs:4-15` maps xmm0/mm0/al/ax/eax all to 0, so any of those names yields `0x50 + n` with no width or class check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Restrict the short form to GP r32 (and handle r16/Sreg separately); reject XMM/MM/r8.
```rust
            Operand::Register(reg) => {
                if is_segment_reg(&reg.name) {
                    /* Sreg forms — see sibling bug */
                }
                let size = reg_size(&reg.name);
                if size == 1 || is_xmm(&reg.name) || is_mm(&reg.name) {
                    return Err(format!("invalid push register {}", reg.name));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                if size == 2 {
                    self.bytes.push(0x66);
                }
                self.bytes.push(0x50 + num);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push_regression_non_gp_xmm0 -- --test-threads=1 --exact
```
**Raw output:**
```text
regression: encode_push must reject non-GP xmm0, got Ok([80])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push_pbt.rs (encode_push_regression_non_gp_xmm0)
