# Bug: encode_neon_add_sub encodes mismatched and reserved arrangements
**Law:** Vector ADD/SUB require matching T in {8B,16B,4H,8H,2S,4S,2D}; 1D (size:Q=11:0) is reserved and must be rejected
**Impact:** The assembler silently encodes `add v0.8b, v0.8b, v0.16b` as `add v0.8b, v0.8b, v0.8b` (source T ignored) and encodes reserved `add v0.1d, v0.1d, v0.1d` as a size=11 Q=0 word, so invalid assembly becomes wrong machine code
**Function:** encode_neon_add_sub
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1164
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_add_sub([v0.8b, v0.8b, v0.16b], is_sub=false)
**Expected:** Err (ARM/gas/llvm-mc require matching T in {8B,16B,4H,8H,2S,4S,2D})
**Actual:** Ok(Word(0x0e208400)) — same as `add v0.8b, v0.8b, v0.8b`. Also Ok for reserved `.1d`.
**Severity:** medium
**Root cause:** neon.rs:1166-1168 discards source arrangements (`let (rn, _)` / `let (rm, _)`) and uses neon_arr_to_q_size which accepts 1d (reserved for integer ADD/SUB)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1166`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Require all three arrangements to match and reject T not in {8b,16b,4h,8h,2s,4s,2d}
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_d != arr_m {
        return Err(format!("add/sub arrangement mismatch: .{arr_d}, .{arr_n}, .{arr_m}"));
    }
    if !matches!(arr_d.as_str(), "8b" | "16b" | "4h" | "8h" | "2s" | "4s" | "2d") {
        return Err(format!("unsupported add/sub arrangement: {arr_d}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_add_sub_regression_mismatched_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_add_sub_pbt::encode_neon_add_sub_neg_invalid_t' (2332199) panicked at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:221:1:
Test failed: invalid/mismatched/reserved T must Err (ARM ADD/SUB T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects add v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:367.
minimal failing input: rd = 0, rn = 0, rm = 0, is_sub = false, td = "8b", tn = "8b", tm = "16b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
