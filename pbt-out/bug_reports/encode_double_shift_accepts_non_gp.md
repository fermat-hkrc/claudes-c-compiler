# Bug: encode_double_shift accepts non-GP registers via reg_num alias

**Law:** SHLD/SHRD source and destination must be general-purpose r/m32 (or r16 with 0x66). Non-GP names (`xmm*`, `mm*`, `st*`) must be rejected; llvm-mc rejects them.
**Impact:** `shldl $1, %xmm0, %edx` encodes as `shldl $1, %eax, %edx` (`xmm0` → encoding 0 via `reg_num`). Public assembler text silently becomes a different instruction — wrong machine code.
**Function:** encode_double_shift
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:927
**Detected by:** Negative/error contract (`encode_double_shift_neg_non_gp`); differential vs llvm-mc rejection
**Minimal input:** `shldl $1, %eax, %xmm0` → Ok(`[0x0f, 0xa4, 0xc0, 0x01]`) which is `shldl $1, %eax, %eax`
**Expected:** `Err`
**Actual:** `Ok([0x0f, 0xa4, 0xc0, 0x01])`
**Severity:** high
**Root cause:** `reg_num` aliases xmm/mm/st onto GP encodings 0–7; `encode_double_shift` never gates with `is_xmm` / GP-only check before using the number.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:927`
```rust
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:** Reject non-GP names before encoding (both Imm and CL arms, and future Mem arms).
```rust
                if is_xmm(&src.name) || is_mm(&src.name) || src.name.starts_with("st")
                    || is_xmm(&dst.name) || is_mm(&dst.name) || dst.name.starts_with("st")
                {
                    return Err("double shift requires GP registers".into());
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_double_shift_regression_xmm0_accepted -- --test-threads=1
```
**Raw output:**
```text
encode_double_shift must reject non-GP xmm0 src, got Ok([15, 164, 194, 1])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs (`encode_double_shift_regression_xmm0_accepted`)
