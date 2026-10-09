# Bug: encode_mov_rr accepts size-mismatched GP register pairs

**Law:** For MOV register-register under a sized mnemonic (`movb`/`movw`/`movl`), both operands must be general-purpose registers whose width matches the mnemonic; mismatched width must be rejected, not silently encoded via `reg_num` width aliasing.
**Impact:** The assembler emits machine code that looks like a valid same-width MOV (e.g. `movl %ax, %ebx` → same bytes as `movl %eax, %ebx`). Callers/inline asm with a wrong-sized name get wrong-width moves with no error, corrupting surrounding register state assumptions.
**Function:** encode_mov_rr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:162
**Detected by:** Negative/Error Contract (and differential agreement that llvm-mc rejects)
**Minimal input:** `movl %ax, %ebx` (also `movb %eax, %bl`, `movl %al, %al`)
**Expected:** `Err(...)` — operand size must match mnemonic (Intel SDM MOV; llvm-mc `-triple=i686` rejects)
**Actual:** `Ok([0x89, 0xc3])` for `movl %ax, %ebx` (identical to `movl %eax, %ebx`); `Ok([0x88, 0xc3])` for `movb %eax, %bl`
**Severity:** high
**Root cause:** `gp_integer.rs:179-191` calls `reg_num` on both names and emits 88/89 without checking `reg_size(src) == size && reg_size(dst) == size`. `reg_num` collapses ax/eax/al to the same 3-bit code.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:179`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;

        if size == 2 {
            self.bytes.push(0x66);
        }
        if size == 1 {
            self.bytes.push(0x88);
        } else {
            self.bytes.push(0x89);
        }
        self.bytes.push(self.modrm(3, src_num, dst_num));
        Ok(())
```
**Suggested fix:** Reject when either register's width disagrees with `size`, before encoding:
```rust
        if reg_size(&src.name) != size || reg_size(&dst.name) != size {
            return Err(format!(
                "mov register size mismatch: src={}, dst={}, expected size {}",
                src.name, dst.name, size
            ));
        }
        // also reject non-GP: only allow names that are true GP at that width
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_rr_regression_rejects_movl_ax_ebx -- --test-threads=1
cargo test --lib encode_mov_rr_neg_mismatched_width -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted size-mismatched `movl %ax, %eax` → [89, c0]; MOV RR requires matching operand size (Intel SDM; llvm-mc rejects). reg_num aliases widths so bytes look like a valid same-width form..
minimal failing input: mode = 0, a_i = 0, b_i = 0

test_encode_mov_rr_regression_rejects_movl_ax_ebx: movl %ax, %ebx must Err (size mismatch); got Ok(Ok([137, 195]))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs (`test_encode_mov_rr_regression_rejects_movl_ax_ebx`, `test_encode_mov_rr_regression_rejects_movb_eax_bl`)
