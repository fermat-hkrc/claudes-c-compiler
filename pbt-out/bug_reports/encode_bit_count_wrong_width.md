# Bug: encode_bit_count accepts r16/r8 as if they were r32

**Law:** ∀ m ∈ {lzcntl,tzcntl,popcntl}, operands must be GP r32 (or r/m32); r16/r8 mixed with *l forms must Err (Intel SDM; llvm-mc rejects)
**Impact:** `lzcntl %ax, %eax` silently emits the same bytes as `lzcntl %eax, %eax` because `reg_num` aliases width variants onto the same 3-bit code. Callers get wrong-width encodings that assemble without error and disassemble as r32 ops.
**Function:** encode_bit_count
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972
**Detected by:** Negative/error contract vs llvm-mc (encode_bit_count_neg_wrong_width)
**Minimal input:** `lzcntl %ax, %eax`
**Expected:** Err (invalid operand width)
**Actual:** Ok([0xf3, 0x0f, 0xbd, 0xc0])
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972-973` calls `reg_num` only; `registers.rs:4-14` maps `ax`/`al` onto the same numbers as `eax` with no width check in encode_bit_count.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(prefix);
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```
**Suggested fix:** Require `reg_size(name) == 4` for both registers (and for mem-dst when added):
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err(format!("{} requires 32-bit GP registers", mnemonic));
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                // ...
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bit_count_regression_rejects_ax_eax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid-width `lzcntl %ax, %eax` → [f3, 0f, bd, c0]; lzcntl is r32-only (Intel SDM; llvm-mc rejects).
minimal failing input: m = "lzcntl", s = "ax", d = "eax", flip = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs (test_encode_bit_count_regression_rejects_ax_eax)
