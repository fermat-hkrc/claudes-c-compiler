# Bug: encode_pop accepts r8/r16/xmm via reg_num aliasing
**Law:** `popl` / `pop` (32-bit default) must only accept r32 GP registers or valid Sreg destinations; r8, r16, and non-GP (xmm/mm/st) names must be rejected (or, for r16, routed through `popw` with 0x66), matching llvm-mc `-triple=i686`.
**Impact:** Invalid operands assemble to the short-form r32 POP bytes (e.g. `popl %xmm0` → `0x58` = `popl %eax`). Wrong machine code is emitted silently when a caller or parser hands a mis-typed register name.
**Function:** encode_pop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:418
**Detected by:** Negative/Error Contract — llvm-mc rejects; Algebraic aliasing via registers.rs reg_num
**Minimal input:** `popl %xmm0` → Ok([0x58]); `popl %al` → Ok([0x58]); `popl %ax` → Ok([0x58])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok([0x58]) — same encoding as `popl %eax`
**Severity:** medium
**Root cause:** The non-segment register arm at `gp_integer.rs:418-421` calls `reg_num` without checking `reg_size` or rejecting xmm/mm/st. `registers.rs:4-16` maps al/ax/eax/xmm0/mm0 all to number 0, so wrong-width and non-GP names silently produce r32 short-form POP.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:418`
```rust
                } else {
                    let num = reg_num(&reg.name).ok_or("bad register")?;
                    self.bytes.push(0x58 + num);
                    Ok(())
                }
```
**Suggested fix:** Require 32-bit GP width and reject non-GP names before encoding.
```rust
                } else {
                    if is_xmm(&reg.name) || is_mm(&reg.name) || reg.name.starts_with("st") {
                        return Err(format!("cannot pop to {}", reg.name));
                    }
                    if reg_size(&reg.name) != 4 {
                        return Err(format!("popl requires r32, got {}", reg.name));
                    }
                    let num = reg_num(&reg.name).ok_or("bad register")?;
                    self.bytes.push(0x58 + num);
                    Ok(())
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop_regression_non_gp_xmm0 -- --test-threads=1
cargo test --lib encode_pop_regression_r8_al -- --test-threads=1
cargo test --lib encode_pop_regression_r16_ax -- --test-threads=1
cargo test --lib encode_pop_neg_xmm -- --test-threads=1
```
**Raw output:**
```text
Test failed: encode_pop must reject non-GP `xmm0`, got Ok([88])
minimal failing input: x = "xmm0"
Test failed: encode_pop must reject r8 `al`, got Ok([88])
minimal failing input: r8 = "al"
Test failed: encode_pop (popl) must reject r16 `ax` (not emit bare 0x58+n), got Ok([88])
minimal failing input: r16 = "ax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_pop_pbt.rs (encode_pop_regression_non_gp_xmm0, encode_pop_regression_r8_al, encode_pop_regression_r16_ax)
