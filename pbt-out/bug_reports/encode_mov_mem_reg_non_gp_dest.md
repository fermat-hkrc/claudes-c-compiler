# Bug: encode_mov_mem_reg accepts non-GP destinations via reg_num aliasing
**Law:** ∀ non-GP dest name (xmm/mm/st/ymm) and valid mem. encode_mov_mem_reg returns Err — opcode 8A/8B is GP-only (Intel SDM); llvm-mc rejects
**Impact:** Forms like `movb (%eax), %xmm0` silently encode as `movb (%eax), %al` because reg_num maps xmm0→0, producing wrong machine code for SIMD-looking assembly
**Function:** encode_mov_mem_reg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:194
**Detected by:** Negative/error contract — non-GP dest vs llvm-mc reject
**Minimal input:** `movb (%eax), %xmm0`
**Expected:** `Err(...)` (bad / non-GP register)
**Actual:** `Ok([0x8a, 0x00])` — same as `movb (%eax), %al`
**Severity:** medium
**Root cause:** gp_integer.rs:195 uses `reg_num` only; `registers.rs:6` includes xmm/mm/st/ymm aliases in the same arms as GP names, so non-GP names encode as GP numbers 0–7 with no class check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:195`
```rust
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;
```
(with alias table)
```rust
        "al" | "ax" | "eax" | "xmm0" | "mm0" | "st" | "st(0)" | "ymm0" => Some(0),
```
**Suggested fix:** Gate destination to true GP names (or reject is_xmm/is_mm/st) before encoding:
```rust
        if is_xmm(&dst.name) || is_mm(&dst.name) || dst.name.starts_with("st") || dst.name.starts_with("ymm") {
            return Err(format!("mov mem→reg destination must be GP, got %{}", dst.name));
        }
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_mem_reg_neg_non_gp_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted non-GP MOV mem→reg `movb (%eax), %xmm0` → [8a, 00]; 8A/8B form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7..
minimal failing input: ni = 0, bi = 0, width = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs (encode_mov_mem_reg_neg_non_gp_dest / KAT-style witness in property)
