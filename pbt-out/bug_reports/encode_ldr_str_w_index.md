# Bug: encode_ldr_str accepts a W index without uxtw/sxtw
**Law:** A W-register index requires an explicit `uxtw` or `sxtw` extend. llvm-mc/gas reject `ldr x0, [x1, w2]`.
**Impact:** `ldr x0, [x1, w2]` silently encodes as `ldr x0, [x1, w2, uxtw]` (0xF8624820). The missing extend is invented rather than rejected.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc rejects W index without extend
**Minimal input:** `encode_ldr_str([Reg("x0"), MemRegOffset{base:"x1", index:"w2", extend:None, shift:None}], is_load=true, size=0b11, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0xF8624820))` — same as `ldr x0, [x1, w2, uxtw]`
**Severity:** medium
**Root cause:** load_store.rs:191-196 defaults a W index with `extend=None` to UXTW, S=0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:201`
```rust
                    if is_w_index {
                        (0b010u32, 0u32) // UXTW, no shift
                    } else {
                        (0b011u32, 0u32) // LSL, no shift
                    }
```
**Suggested fix:** Reject a W index when no extend is specified; keep the X-index LSL default.
```rust
                    if is_w_index {
                        return Err("ldr/str: W index requires uxtw or sxtw".to_string());
                    } else {
                        (0b011u32, 0u32) // LSL, no shift
                    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_w_index -- --test-threads=1
```
**Raw output:**
```text
LDR X0, [X1, W2] must Err; W index requires uxtw/sxtw (llvm-mc rejects it)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
