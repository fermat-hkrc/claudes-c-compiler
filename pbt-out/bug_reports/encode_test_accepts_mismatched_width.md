# Bug: encode_test accepts size-mismatched GP register pairs
**Law:** TEST RR operands must match the mnemonic width (testl→r32×r32, testw→r16×r16, testb→r8×r8); mismatched pairs llvm-mc rejects must return Err.
**Impact:** `testl %ax, %ebx` silently encodes as if both were 32-bit (`testl %eax, %ebx` bytes), producing wrong object code when a caller/parser feeds mixed-width names.
**Function:** encode_test
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:654
**Detected by:** Negative/error contract — llvm-mc rejection differential
**Minimal input:** `testl %ax, %eax` (also `testl %ax, %ebx`)
**Expected:** `Err(...)`
**Actual:** `Ok([0x85, 0xc0])` (same bytes as testl %eax, %eax)
**Severity:** medium
**Root cause:** RR arm calls `reg_num` only; `reg_num` aliases ax/eax/al to the same number with no `reg_size` gate against mnemonic width.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:654`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x84 } else { 0x85 });
                self.bytes.push(self.modrm(3, src_num, dst_num));
                Ok(())
            }
```
**Suggested fix:** Gate both registers on mnemonic size before encoding:
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != Some(size) || reg_size(&dst.name) != Some(size) {
                    return Err(format!("size-mismatched test operands"));
                }
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                // ... existing encode ...
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_test_regression_mismatched_width_testl_ax_ebx -- --test-threads=1
```
**Raw output:**
```text
regression: size-mismatched TEST must Err, got Ok([133, 195])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_test_pbt.rs (encode_test_regression_mismatched_width_testl_ax_ebx)
