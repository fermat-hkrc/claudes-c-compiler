# Bug: encode_double_shift accepts width-mismatched GP registers

**Law:** For `shldl`/`shrdl` (size=4 dispatch), source and destination must be 32-bit GP registers. r16/r8 names must be rejected (llvm-mc rejects `shldl $1, %ax, %edx`); 16-bit forms need a distinct `shldw` path with 0x66.
**Impact:** `shldl $1, %ax, %edx` encodes as `shldl $1, %eax, %edx` because `reg_num("ax") == reg_num("eax")`. Width-mismatched assembler text silently becomes a 32-bit double-shift.
**Function:** encode_double_shift
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:927
**Detected by:** Negative/error contract (`encode_double_shift_neg_mismatched_width`); differential vs llvm-mc rejection
**Minimal input:** `shldl $1, %eax, %ax` → Ok(`[0x0f, 0xa4, 0xc0, 0x01]`) same as `%eax` dst
**Expected:** `Err`
**Actual:** `Ok([0x0f, 0xa4, 0xc0, 0x01])`
**Severity:** high
**Root cause:** No `reg_size(name) == 4` (or size parameter) check on src/dst; `_size` is unused; `reg_num` collapses ax/eax to the same encoding.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:927`
```rust
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:** Gate register operands on `reg_size` matching the requested size (callers pass 4 today); reject r8/r16 under shldl/shrdl.
```rust
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err("double shift register width mismatch".into());
                }
                // still reject non-GP (reg_size defaults unknown names to 4)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_double_shift_regression_mismatched_width -- --test-threads=1
```
**Raw output:**
```text
encode_double_shift must reject width-mismatched `shldl $1, %eax, %ax`, got Ok([15, 164, 192, 1])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs (`encode_double_shift_regression_mismatched_width`)
