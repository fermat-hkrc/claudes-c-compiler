# Bug: encode_mov accepts mismatched NEON arrangements
**Law:** `mov v0.16b, v1.8b` is invalid; llvm-mc/gas require matching 8b or 16b arrangements
**Impact:** A 128-bit dest with an 8-bit-arrangement source is encoded as ORR V0.16B, V1.16B, V1.16B, silently widening the source
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:13
**Detected by:** Negative/error contract — llvm-mc arrangement-mismatch rejection
**Minimal input:** encode_mov([RegArrangement { v0, 16b }, RegArrangement { v1, 8b }])
**Expected:** Err
**Actual:** Ok(Word(0x4ea01c00)) — ORR V0.16B, V0.16B, V0.16B-style Q=1 encoding using rm=0
**Severity:** medium
**Root cause:** the source arrangement is bound as `_arr_m` and never compared to `arr_d`; Q is taken only from the destination
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:13`
```rust
            Some(Operand::RegArrangement { reg: rm_name, arrangement: _arr_m })) =
        (operands.first(), operands.get(1))
```
**Suggested fix:** Require `arr_d == arr_m` and restrict T to 8b/16b.
```rust
            Some(Operand::RegArrangement { reg: rm_name, arrangement: arr_m })) =
        (operands.first(), operands.get(1))
    {
        if arr_d != arr_m || (arr_d != "8b" && arr_d != "16b") {
            return Err("mov vector arrangements must match 8b or 16b".to_string());
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_arr_mismatch -- --test-threads=1
```
**Raw output:**
```text
16b vs 8b must Err, got Ok(Word(1319115776))
minimal failing input: vd = 0, vn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
