# Bug: encode_mov_rr accepts non-GP registers (xmm/mm/st) as GP MOV

**Law:** Opcodes 0x88/0x89 MOV r/r encode general-purpose registers only. Names that are not GP at the mnemonic width (xmm*, mm*, st*) must be rejected for `movb`/`movw`/`movl` RR form.
**Impact:** `movl %xmm0, %eax` is encoded as `movl %eax, %eax` (`[0x89, 0xc0]`) because `reg_num("xmm0") == Some(0)`. Silent wrong code generation for mistyped or mis-parsed operands.
**Function:** encode_mov_rr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:162
**Detected by:** Negative/Error Contract (llvm-mc rejects; SUT accepts)
**Minimal input:** `movl %xmm0, %eax` (also `movb %al, %xmm0`)
**Expected:** `Err(...)` — non-GP operand not valid for 88/89 GP MOV
**Actual:** `Ok([0x89, 0xc0])` for `movl %xmm0, %eax`; `Ok([0x88, 0xc0])` for `movb %al, %xmm0`
**Severity:** high
**Root cause:** `gp_integer.rs:179-180` uses `reg_num`, which maps xmm/mm/st/ymm onto 0–7 (`registers.rs:4-15`), with no GP-only gate before the 88/89 path.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:179`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;
```
**Suggested fix:** After segment arms, require true GP names at the requested width (e.g. match on explicit GP sets, or `reg_size` + reject xmm/mm/st prefixes):
```rust
        fn is_gp_at(name: &str, size: u8) -> bool {
            reg_size(name) == size
                && !name.starts_with("xmm")
                && !name.starts_with("ymm")
                && !name.starts_with("mm")
                && !name.starts_with("st")
        }
        if !is_gp_at(&src.name, size) || !is_gp_at(&dst.name, size) {
            return Err(format!("bad GP register for mov size {size}: {}, {}", src.name, dst.name));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_rr_regression_rejects_xmm -- --test-threads=1
cargo test --lib encode_mov_rr_neg_non_gp -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted non-GP MOV RR `movb %al, %xmm0` → [88, c0]; 88/89 form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7..
minimal failing input: ni = 0, gi = 0, width = 1, non_gp_as_src = false

test_encode_mov_rr_regression_rejects_xmm: movl %xmm0, %eax must Err; got Ok(Ok([137, 192]))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs (`test_encode_mov_rr_regression_rejects_xmm`)
