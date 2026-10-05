# Bug: encode_neon_fcvtn ignores a third operand
**Law:** A third operand on FCVTN/FCVTN2 must be rejected (gas/llvm-mc reject it; README claims gas compatibility)
**Impact:** Trailing junk after FCVTN is silently dropped, so a mistyped extra register does not fail the assemble
**Function:** encode_neon_fcvtn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1652
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_fcvtn([v0.4h, v0.4s, v0.4h], is_high=false)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0x0e216800)) — same encoding as `fcvtn v0.4h, v0.4s`
**Severity:** medium
**Root cause:** neon.rs:1653–1654 read only operands[0] and operands[1]; there is no maximum-arity check, so length 3 is accepted and operands[2] is never read
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1653`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Reject anything other than exactly 2 operands
```rust
    if operands.len() != 2 {
        return Err("fcvtn requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn_neg_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3 operands must Err (llvm-mc rejects fcvtn v0.4h, v0.4s, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs:348.
minimal failing input: rd = 0, rn = 0, extra = 0, pair = (
    "4h",
    "4s",
    false,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
