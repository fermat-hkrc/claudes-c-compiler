# Bug: encode_alu accepts size-mismatched GP register pairs via reg_num aliasing
**Law:** ∀ ALU mnemonic with size suffix w, both register operands must have width w; pairs llvm-mc rejects (e.g. `addl %ax, %ebx`) must return Err.
**Impact:** Assembler silently encodes invalid AT&T as a same-width form (ax→eax via reg_num), producing machine code the programmer did not write — hard-to-diagnose miscompilation if a frontend ever feeds mixed-width names.
**Function:** encode_alu
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:496
**Detected by:** Negative/error contract (llvm-mc reject set)
**Minimal input:** `addl %ax, %ebx` (also `addl %ax, %eax`)
**Expected:** Err(...)
**Actual:** Ok(`[0x01, 0xc3]`) — same bytes as `addl %eax, %ebx`
**Severity:** high
**Root cause:** RR arm uses only `reg_num` (name→0..7) and never checks `reg_size` against the mnemonic size suffix, so ax/eax/al collide.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:496`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;

                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x00 } else { 0x01 } + alu_op * 8);
                self.bytes.push(self.modrm(3, src_num, dst_num));
                Ok(())
            }
```
**Suggested fix:** Gate with `reg_size(&name) == size` (or mnemonic width) before encoding; return Err on mismatch.
```rust
                if reg_size(&src.name) != size || reg_size(&dst.name) != size {
                    return Err(format!("register size mismatch for {mnemonic}"));
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib 'backend::i686::assembler::encoder::encode_alu_pbt::encode_alu_regression_mismatched_width_addl_ax_ebx' -- --test-threads=1 --nocapture
```
**Raw output:**
```text
regression: size-mismatched ALU must Err, got Ok([1, 195])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_alu_pbt.rs (encode_alu_regression_mismatched_width_addl_ax_ebx)
