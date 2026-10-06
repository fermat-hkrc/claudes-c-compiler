# Bug: encode_ldp_stp accepts a 32-bit W register as the memory base
**Law:** LDP/STP base must be a 64-bit Xn or SP; a W register is not a valid addressing base
**Impact:** `stp w0, w0, [w0]` is encoded as STP W0, W0, [X0], silently widening the base
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("w0"), Reg("w0"), Mem{base:"w0", offset:0}], is_load=false)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) with Rn=0 (X0)
**Severity:** medium
**Root cause:** load_store.rs:505 `parse_reg_num` accepts the 'w' prefix and returns the number; encode_ldp_stp does not require an X/SP base
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:505`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Require the base name to be Xn, SP, or LR
```rust
    let b = base.to_lowercase();
    if !(b.starts_with('x') || b == "sp" || b == "lr") {
        return Err("ldp/stp base must be Xn|SP".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_w_base -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): [..., "W base", ...]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
