# Bug: encode_neon_two_misc encodes opcode-reserved arrangements
**Law:** ARM two-misc reserves opcode-specific arrangements (vector ABS/NEG/SQABS/SQNEG 1D; CLS/CLZ 2D; REV16 not-byte; REV32 not-byte/half); those must be rejected
**Impact:** `abs v0.1d, v0.1d` and `cls v0.2d, v0.2d` are encoded as reserved size/Q encodings instead of diagnosed. llvm-mc/gas reject the same assembly
**Function:** encode_neon_two_misc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1407
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_two_misc([v0.1d, v0.1d], u_bit=0, opcode=0b01011)
**Expected:** Err (llvm-mc/gas reject vector ABS .1d)
**Actual:** Ok(Word) with Q=0 size=11
**Severity:** medium
**Root cause:** neon.rs:1410 uses neon_arr_to_q_size, which maps 1d/2d, with no opcode-specific arrangement check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1410`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject arrangements that ARM reserves for the given opcode
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    let reserved = match (opcode, u_bit, arr_d.as_str()) {
        (0b01011, _, "1d") => true,           // ABS/NEG
        (0b00111, _, "1d") => true,           // SQABS/SQNEG
        (0b00100, _, "1d" | "2d") => true,    // CLS/CLZ
        (0b00001, 0, t) if t != "8b" && t != "16b" => true, // REV16
        (0b00000, 1, "2s" | "4s" | "1d" | "2d") => true,    // REV32
        _ => false,
    };
    if reserved {
        return Err(format!("two-misc: reserved arrangement {} for opcode {:05b}", arr_d, opcode));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_reserved_1d -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_neg_reserved_t' (2372514) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: reserved T must Err (llvm-mc rejects abs v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:513.
minimal failing input: rd = 0, rn = 0, case = (
    "abs",
    0,
    11,
    "1d",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
