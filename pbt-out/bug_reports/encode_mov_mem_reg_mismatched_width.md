# Bug: encode_mov_mem_reg accepts mismatched dest width via reg_num aliasing
**Law:** ∀ mnemonic width w ∈ {1,2,4} and dest register r with reg_size(r) ≠ w. encode_mov_mem_reg(mem, r, w) returns Err (Intel MOV r,m requires matching operand size; llvm-mc rejects)
**Impact:** Size-mismatched forms such as `movl (%eax), %ax` silently encode as a 32-bit load into the low 3-bit register number of ax (same as eax), producing machine code that does not match the written assembly width
**Function:** encode_mov_mem_reg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:194
**Detected by:** Negative/error contract — mismatched width vs llvm-mc reject
**Minimal input:** `movl (%eax), %ax`
**Expected:** `Err(...)` (invalid operand size)
**Actual:** `Ok([0x8b, 0x00])` — same bytes as `movl (%eax), %eax`
**Severity:** medium
**Root cause:** gp_integer.rs:195 takes `reg_num(&dst.name)` with no `reg_size` check against the mnemonic `size`; `registers.rs:4-15` maps ax/eax (and al) to the same number 0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:195`
```rust
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;
```
**Suggested fix:** Reject when dest width disagrees with the mnemonic size before encoding:
```rust
        if reg_size(&dst.name) != size {
            return Err(format!(
                "mov mem→reg size mismatch: dest %{} is {}-bit, mnemonic wants {}-bit",
                dst.name, reg_size(&dst.name), size
            ));
        }
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_mem_reg_neg_mismatched_width -- --test-threads=1
cargo test --lib encode_mov_mem_reg_regression_mismatched_width_ax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted size-mismatched mem→reg `movl (%eax), %ax` → [8b, 00]; MOV r,m requires matching operand size (Intel SDM; llvm-mc rejects). reg_num aliases widths so bytes look like a valid same-width form..
minimal failing input: mode = 0, bi = 0, di = 0, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs (encode_mov_mem_reg_regression_mismatched_width_ax)
