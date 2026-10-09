# Bug: encode_pop16 accepts r32/r8 GP names via reg_num aliasing
**Law:** ∀ r ∉ r16_gp. encode_pop16([r]) must be Err. Intel POP with 16-bit operand size and AT&T `popw` accept only r16 (and Sreg/memory); llvm-mc rejects `popw %eax` and `popw %al`.
**Impact:** `popw %eax` / `popw %al` silently encode as `popw %ax` (`[0x66, 0x58]`), masking assembler typos and producing wrong-width code without diagnostic.
**Function:** encode_pop16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:325
**Detected by:** Negative/error contract vs llvm-mc rejection
**Minimal input:** `popw %eax` → SUT `Ok([0x66, 0x58])`, llvm-mc error `invalid operand`
**Expected:** `Err(...)`
**Actual:** `Ok([0x66, 0x58])` (same for `%al` → same bytes as `%ax`)
**Severity:** medium
**Root cause:** system.rs:341 calls `reg_num` which aliases al/ax/eax → 0 (registers.rs:4-15) with no `reg_size == 2` gate on the non-segment arm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:340`
```rust
                } else {
                    let num = reg_num(&reg.name).ok_or("bad register")?;
                    self.bytes.push(0x66);
                    self.bytes.push(0x58 + num);
                    Ok(())
                }
```
**Suggested fix:** Require 16-bit GP names before encoding:
```rust
                } else {
                    if reg_size(&reg.name) != 2 {
                        return Err(format!("popw requires r16 register, got {}", reg.name));
                    }
                    let num = reg_num(&reg.name).ok_or("bad register")?;
                    self.bytes.push(0x66);
                    self.bytes.push(0x58 + num);
                    Ok(())
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop16_neg_r32 -- --test-threads=1
cargo test --lib test_encode_pop16_regression_r32_accepted -- --test-threads=1
```
**Raw output:**
```text
Test failed: popw %eax must be Err like llvm-mc, got Ok([66, 58])
minimal failing input: r32 = "eax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_pop16_pbt.rs::test_encode_pop16_regression_r32_accepted
