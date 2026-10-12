# Bug: unsuffixed cmovcc with r16 operands omits 0x66 operand-size prefix
**Law:** Unsuffixed AT&T `cmovcc %r16, %r16` (and mem→r16) must encode as 16-bit CMOVcc with leading 0x66, matching llvm-mc/GAS size inference from register width. Emitting bare 32-bit 0F 40+cc for r16 names is wrong machine code.
**Impact:** Compilers/assemblers that emit unsuffixed `cmove %ax, %bx` (common when operand size is carried by the register names) produce instructions that actually operate on the full 32-bit registers without the override — silent wrong-width conditional moves.
**Function:** encode_cmovcc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1118
**Detected by:** Differential — llvm-mc i686 (encode_cmovcc_diff_unsuffixed_r16_size)
**Minimal input:** `cmove %ax, %ax` → SUT `[0f, 44, c0]`; llvm-mc `[66, 0f, 44, c0]`
**Expected:** `[0x66, 0x0f, 0x44, 0xc0]`
**Actual:** `[0x0f, 0x44, 0xc0]`
**Severity:** high
**Root cause:** `is_16bit` is derived only from an explicit `w` mnemonic suffix (`gp_integer.rs:1104-1119`). The inline comment defaults unsuffixed mnemonics to 32-bit, but the register arm still accepts r16 names via `reg_num` and emits without 0x66 — neither rejecting nor size-inferring.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1118`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if is_16bit { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```
**Suggested fix:** Derive width from destination (and require matching source) register size, or require a matching suffix; at minimum emit 0x66 when both GP regs are 16-bit:
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_sz = reg_size(&src.name);
                let dst_sz = reg_size(&dst.name);
                if src_sz != dst_sz || (src_sz != 2 && src_sz != 4) {
                    return Err("cmovcc requires matching r16/r32 GP registers".into());
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                let use_16 = is_16bit || dst_sz == 2;
                if use_16 { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cmovcc_regression_unsuffixed_r16_prefix -- --test-threads=1 --nocapture
```
**Raw output:**
```text
assertion `left == right` failed: cmove %ax, %bx must be [66, 0f, 44, d8], got [0f, 44, d8]
  left: [15, 68, 216]
 right: [102, 15, 68, 216]
minimal failing input (PBT): m = "cmove", src = "ax", dst = "ax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs (test_encode_cmovcc_regression_unsuffixed_r16_prefix)
