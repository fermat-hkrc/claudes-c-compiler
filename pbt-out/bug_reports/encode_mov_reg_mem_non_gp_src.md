# Bug: encode_mov_reg_mem accepts non-GP source via reg_num alias
**Law:** ∀ bad ∈ {xmm*,mm*,st*,ymm*}, mem, width. encode_mov_reg_mem(bad, mem, width) = Err(_)
**Impact:** Assembler silently treats `movl %xmm0, (%eax)` as `movl %eax, (%eax)` (`[89, 00]`), emitting GP store bytes for a non-GP register name
**Function:** encode_mov_reg_mem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:216
**Detected by:** Negative/error contract — llvm-mc rejects non-GP src for 88/89 MOV
**Minimal input:** `movb %xmm0, (%eax)` (also `movl %xmm0, (%eax)`)
**Expected:** `Err(...)` (llvm-mc rejects)
**Actual:** `Ok([0x88, 0x00])` for movb / `Ok([0x89, 0x00])` for movl — same as al/eax store
**Severity:** high
**Root cause:** `reg_num` (registers.rs:4-14) maps xmm0/mm0/st/ymm0 onto GP numbers 0–7; encode_mov_reg_mem never checks `is_xmm`/`is_mm`/st/ymm before using that number
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:217`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Suggested fix:** Reject non-GP names before reg_num:
```rust
        if is_xmm(&src.name) || is_mm(&src.name) || src.name.starts_with("st") || src.name.starts_with("ymm") {
            return Err(format!("non-GP register for mov: {}", src.name));
        }
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_reg_mem_neg_non_gp_src -- --test-threads=1
cargo test --lib encode_mov_reg_mem_regression_non_gp_xmm0 -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted non-GP MOV reg→mem `movb %xmm0, (%eax)` → [88, 00]; 88/89 form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7..
minimal failing input: ni = 0, bi = 0, width = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs (encode_mov_reg_mem_regression_non_gp_xmm0)
