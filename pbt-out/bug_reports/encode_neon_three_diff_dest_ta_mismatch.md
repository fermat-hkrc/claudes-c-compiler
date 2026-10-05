# Bug: encode_neon_three_diff discards destination arrangement
**Law:** LONG three-different instructions have a wider destination than the source: Ta must be widen(Tb). encode_neon_three_diff([Vd.Td, Vn.Tb, Vm.Tb], …) must Err when Td ≠ widen(Tb), matching llvm-mc
**Impact:** `saddl v0.8b, v0.8b, v0.8b` (dest not 8h) is encoded as if dest were 8h. Wrong-arrangement dest is silently accepted and the Q/size fields still come only from Vn.
**Function:** encode_neon_three_diff
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:89
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_three_diff([v0.8b, v0.8b, v0.8b], u_bit=0, opcode=0, is_high=false)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) — dest arrangement is bound to `_arr_d` and unused
**Severity:** medium
**Root cause:** neon.rs:93 extracts Rd from dest but discards its arrangement; Q/size come only from Vn.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:93`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest Ta to be the widening of source Tb (8b/16b→8h, 4h/8h→4s, 2s/4s→2d).
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let expected_ta = match arr_n.as_str() {
        "8b" | "16b" => "8h",
        "4h" | "8h" => "4s",
        "2s" | "4s" => "2d",
        _ => return Err(format!("unsupported source arrangement for three-diff: {}", arr_n)),
    };
    if arr_d != expected_ta {
        return Err(format!("destination arrangement {} does not widen {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_dest_ta_mismatch -- --test-threads=1
```
**Raw output:**
```text
Test failed: dest Ta must be widen(Tb); llvm-mc rejects saddl v0.8b, v0.8b, v0.8b
minimal failing input: rd = 0, rn = 0, rm = 0, tb = "8b", td = "8b", insn = (0, 0, "saddl")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
