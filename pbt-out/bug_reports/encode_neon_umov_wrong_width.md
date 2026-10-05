# Bug: encode_neon_umov accepts the wrong GPR width for UMOV
**Law:** ARM UMOV requires Wd for Ts in {B,H,S} and Xd for Ts=D; encode_neon_umov([wrong-width Rd, Vn.Ts[i]]) = Err
**Impact:** `umov x0, v0.b[0]` is encoded with Q=1 (a reserved/invalid UMOV pairing), so a width typo silently produces a word llvm-mc and gas refuse to assemble.
**Function:** encode_neon_umov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:461
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_umov([x0, v0.b[0]])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x4e013c00)) — Q=1 with .B element
**Severity:** medium
**Root cause:** neon.rs:471 sets Q from dest GPR width (`is_64`) and never checks that Wd pairs with B/H/S and Xd pairs with D.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:471`
```rust
            let q = if is_64 { 1u32 } else { 0 };
```
**Suggested fix:** Derive Q from the element size and reject a dest width that does not match.
```rust
            let q = match elem_size.as_str() {
                "b" | "h" | "s" => {
                    if is_64 { return Err("umov B/H/S requires a W register".into()); }
                    0u32
                }
                "d" => {
                    if !is_64 { return Err("umov D requires an X register".into()); }
                    1u32
                }
                _ => return Err(format!("unsupported umov element size: {}", elem_size)),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_wrong_width -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_wrong_width' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:600:9:
umov x0, v0.b[0] must Err (llvm-mc requires Wd for Ts=B)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
