# Bug: encode_ldp_stp encodes writeback when Rn is also Rt1 or Rt2
**Law:** Pre/post-index LDP/STP with Rn in {Rt1, Rt2} and Rn != SP is ARM UNPREDICTABLE; llvm-mc rejects it
**Impact:** `stp w0, w0, [x0, #4]!` is encoded instead of rejected, producing an unpredictable writeback pair store
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("w0"), Reg("w0"), MemPreIndex{base:"x0", offset:4}], is_load=false)
**Expected:** Err (llvm-mc: unpredictable STP instruction, writeback base is also a source)
**Actual:** Ok(Word) with pre-index mode bits 011
**Severity:** medium
**Root cause:** load_store.rs:488-492 (and the post-index arm) encode MemPreIndex/MemPostIndex with no check that Rn is distinct from Rt1/Rt2 when Rn != 31
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:488`
```rust
        Some(Operand::MemPreIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
```
**Suggested fix:** After parsing rn, reject writeback overlap unless rn == 31 (SP)
```rust
            if rn != 31 && (rn == rt1 || rn == rt2) {
                return Err("ldp/stp writeback base overlaps Rt".to_string());
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_writeback_overlap -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): [..., "writeback Rn==Rt1"]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
