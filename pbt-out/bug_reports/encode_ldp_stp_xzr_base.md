# Bug: encode_ldp_stp accepts XZR/x31 as the memory base
**Law:** LDP/STP base Rn is Xn|SP; register 31 in the Rn field is SP, not XZR, so XZR/x31 is not a valid base
**Impact:** `stp w0, w0, [xzr]` is encoded as STP W0, W0, [SP], silently targeting the stack pointer
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("w0"), Reg("w0"), Mem{base:"xzr", offset:0}], is_load=false)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) with Rn=31 (SP)
**Severity:** medium
**Root cause:** load_store.rs:505 `parse_reg_num(base)` maps "xzr"/"x31" to 31, the SP encoding in this instruction class
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:505`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Reject XZR/WZR/x31/w31 as the base; only Xn and SP
```rust
    let base_l = base.to_lowercase();
    if matches!(base_l.as_str(), "xzr" | "wzr" | "x31" | "w31") {
        return Err("ldp/stp base cannot be XZR".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_xzr_base -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): [..., "XZR base", "x31 base", ...]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
