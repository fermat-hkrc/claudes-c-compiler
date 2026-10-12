# Bug: encode_bsr_bsf accepts r16/r8 via reg_num aliasing
**Law:** `bsfl`/`bsrl` encode r32, r/m32 only (Intel SDM). Operands whose `reg_size` is not 4 must be rejected; llvm-mc rejects them.
**Impact:** Wrong-width register names (`%ax`, `%al`, …) are accepted and encoded as the aliased r32 form, so a typo or size-mismatch in assembly silently produces a 32-bit bit-scan instead of an assembler error — incorrect machine code with no diagnostic.
**Function:** encode_bsr_bsf
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:996
**Detected by:** Negative/error contract — llvm-mc rejection + SUT must Err (encode_bsr_bsf_neg_wrong_width)
**Minimal input:** `bsfl %eax, %ax` (also `bsfl %ax, %ebx`, `bsrl %eax, %al`)
**Expected:** `Err(...)`
**Actual:** `Ok([0x0f, 0xbc, 0xc0])` — same bytes as `bsfl %eax, %eax`
**Severity:** medium
**Root cause:** gp_integer.rs:996-1001 Register arm calls `reg_num` only; `registers.rs:4` maps ax/al/xmm0/… onto the same 3-bit number as eax, with no `reg_size == 4` gate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:996`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```
**Suggested fix:** Require 32-bit GP width on both registers (and on the memory-form dest) before encoding.
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err(format!("bsf/bsr requires r32 operands"));
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bsr_bsf_regression_r16_src_accepted -- --test-threads=1
cargo test --lib test_encode_bsr_bsf_regression_r8_dst_accepted -- --test-threads=1
```
**Raw output:**
```text
regression: bsfl %ax, %ebx must be Err, got Ok([0f, bc, d8])
regression: bsrl %eax, %al must be Err, got Ok([0f, bd, c0])
Test failed: `bsfl %eax, %ax` must be Err like llvm-mc, got Ok([0f, bc, c0])
minimal failing input: mnemonic = "bsfl", bad = "ax", good = "eax", bad_is_src = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs (test_encode_bsr_bsf_regression_r16_src_accepted, test_encode_bsr_bsf_regression_r8_dst_accepted)
