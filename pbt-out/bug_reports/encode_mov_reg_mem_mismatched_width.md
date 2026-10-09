# Bug: encode_mov_reg_mem accepts size-mismatched GP source
**Law:** ∀ mnemonic width w ∈ {1,2,4}, src with reg_size(src)≠w, valid mem. encode_mov_reg_mem(src, mem, w) = Err(_)
**Impact:** Assembler silently encodes wrong-width forms such as `movl %ax, (%eax)` as if the source were eax (`[89, 00]`), producing incorrect machine code instead of a diagnostic
**Function:** encode_mov_reg_mem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:216
**Detected by:** Negative/error contract — llvm-mc rejects mismatched width
**Minimal input:** `movl %ax, (%eax)`
**Expected:** `Err(...)` (llvm-mc: invalid operand for instruction)
**Actual:** `Ok([0x89, 0x00])` — same bytes as `movl %eax, (%eax)`
**Severity:** high
**Root cause:** gp_integer.rs:217 calls `reg_num` which aliases ax/al/eax to the same 3-bit encoding, with no `reg_size(&src.name) == size` gate before encoding
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:217`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Suggested fix:** Gate on register width before encoding:
```rust
        if reg_size(&src.name) != size {
            return Err(format!("register size mismatch for mov: {}", src.name));
        }
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_reg_mem_neg_mismatched_width -- --test-threads=1
cargo test --lib encode_mov_reg_mem_regression_mismatched_width_ax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted size-mismatched reg→mem `movl %ax, (%eax)` → [89, 00]; MOV m,r requires matching operand size (Intel SDM; llvm-mc rejects). reg_num aliases widths so bytes look like a valid same-width form..
minimal failing input: mode = 0, bi = 0, si = 0, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs (encode_mov_reg_mem_regression_mismatched_width_ax)
