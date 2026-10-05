# Bug: encode_neon_elem_long ignores destination arrangement
**Law:** Long by-element SMULL/UMULL/SMLAL/… require dest Ta to be the widened form of source Tb (4h/8h→4s, 2s/4s→2d); a mismatched dest must be rejected
**Impact:** Illegal assembly such as `smull v0.8b, v0.4h, v0.h[0]` is encoded as if the dest were `.4s`, so a wrong arrangement is assembled instead of diagnosed
**Function:** encode_neon_elem_long
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:235
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem_long([v0.8b, v0.4h, v0.h[0]], u=0, opcode=0b1010, is_high=false)
**Expected:** Err
**Actual:** Ok(Word) — dest arrangement discarded; Q/size taken only from the source
**Severity:** medium
**Root cause:** neon.rs:239 binds dest arrangement as `_arr_d` and never checks it against the mandated widened Ta
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:239`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Compare dest arrangement with the mandated widened Ta for the source (4h/8h → 4s, 2s/4s → 2d) and return Err on mismatch
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    // after size/Q from arr_n:
    let mandated = match arr_n.as_str() {
        "4h" | "8h" => "4s",
        "2s" | "4s" => "2d",
        _ => unreachable!(),
    };
    if arr_d != mandated {
        return Err(format!("elem-long dest arrangement {} does not match source {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_mismatch_ta -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_long_pbt::test_encode_neon_elem_long_regression_mismatch_ta' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs:529:5:
smull v0.8b, v0.4h, v0.h[0] must Err (llvm-mc/gas require dest .4s for .4h source)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
