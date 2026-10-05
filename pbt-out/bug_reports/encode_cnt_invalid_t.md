# Bug: encode_cnt encodes illegal arrangements as CNT .8b
**Law:** CNT is only valid for T in {8b, 16b}; any other arrangement must be rejected
**Impact:** `cnt v0.4h, v0.4h` (and 8h/2s/4s/2d/1d/…) is assembled as `cnt v0.8b, v0.8b` (0x0e205800), emitting the wrong instruction instead of an error
**Function:** encode_cnt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:23
**Detected by:** Negative/Error Contract
**Minimal input:** encode_cnt([v0.4h, v0.4h])
**Expected:** Err (llvm-mc: invalid operand; gas: operand mismatch; comment: only .8b/.16b)
**Actual:** Ok(Word(0x0e205800)) — Q=0 size=00, i.e. CNT .8b
**Severity:** high
**Root cause:** neon.rs:33 sets Q=1 only for `"16b"` and Q=0 for every other arrangement, with no check that T is 8b or 16b
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:33`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 }; // .8b -> Q=0, .16b -> Q=1
```
**Suggested fix:** Reject any arrangement other than 8b/16b
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("cnt: unsupported arrangement .{}, expected .8b or .16b", arr_d));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_cnt_pbt::encode_cnt_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:166:1:
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects cnt v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:274.
minimal failing input: rd = 0, rn = 0, t = "4h"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
