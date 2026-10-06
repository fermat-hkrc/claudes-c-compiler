# Bug: encode_mov encodes mov v0.4s, v1.4s with Q=0
**Law:** GNU as rejects vector MOV except `8b`/`16b`; llvm-mc accepts `4s` as the 16-byte form (Q=1). Encoding Q=0 is wrong under both contracts
**Impact:** `mov v0.4s, v1.4s` is assembled as the 8-byte ORR (Q=0), so a 128-bit vector move only updates the low 64 bits
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:18
**Detected by:** Negative/error contract — GNU as 8b/16b-only vector MOV
**Minimal input:** encode_mov([RegArrangement { v0, 4s }, RegArrangement { v1, 4s }])
**Expected:** Err (gas) or Word(0x4ea11c20) (llvm-mc Q=1 16b alias)
**Actual:** Ok(Word(0x0ea01c00)) — Q=0 8-byte ORR
**Severity:** high
**Root cause:** Q is 1 only when `arr_d == "16b"`, so 4s/8h/2d (128-bit) get Q=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:18`
```rust
        let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Follow gas: only 8b (Q=0) and 16b (Q=1); reject other arrangements.
```rust
        let q: u32 = match arr_d.as_str() {
            "16b" => 1,
            "8b" => 0,
            _ => return Err(format!("mov vector arrangement must be 8b or 16b, got {arr_d}")),
        };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_vec_4s -- --test-threads=1
```
**Raw output:**
```text
gas-invalid 4s vector mov must Err, got Ok(Word(245373952))
minimal failing input: vd = 0, vn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
