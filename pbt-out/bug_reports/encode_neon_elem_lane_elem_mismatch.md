# Bug: encode_neon_elem ignores RegLane elem_size
**Law:** The lane element size of Vm.Ts[index] must match arrangement T (4h/8h → h, 2s/4s → s); a mismatched elem_size must be rejected
**Impact:** `mul v0.4h, v0.4h, v0.b[0]` encodes as a valid .h-lane by-element word, so illegal assembly (llvm-mc: invalid operand) becomes a 32-bit instruction
**Function:** encode_neon_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1591
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem([v0.4h, v0.4h, v0.b[0]], u=0, opcode=0b1000)
**Expected:** Err
**Actual:** Ok(Word) — elem_size discarded
**Severity:** medium
**Root cause:** neon.rs:1595-1596 matches `Operand::RegLane { reg, index, .. }`, dropping `elem_size`, then H:L:M come only from dest size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1595`
```rust
        Operand::RegLane { reg, index, .. } => (parse_reg_num(reg).ok_or("invalid reg")?, *index),
```
**Suggested fix:** Bind and check elem_size against dest size
```rust
        Operand::RegLane { reg, elem_size, index } => {
            let rm = parse_reg_num(reg).ok_or("invalid reg")?;
            (rm, elem_size.clone(), *index)
        }
```
then after `(q, size) = neon_arr_to_q_size(&arr_d)?`:
```rust
    let expect = match size { 0b01 => "h", 0b10 => "s", _ => "" };
    if elem_size != expect {
        return Err(format!("lane elem {elem_size} does not match {arr_d}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_lane_elem_mismatch -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::encode_neon_elem_neg_lane_elem_mismatch' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:223:1:
Test failed: lane elem_size b must match arrangement h (llvm-mc rejects mul v0.4h, v0.4h, v0.b[0])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
