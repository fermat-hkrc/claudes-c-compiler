# Bug: encode_neon_float_elem ignores a fourth operand
**Law:** A fourth operand on FMUL/FMLA/FMLS/FMULX by-element must be rejected (gas/llvm-mc reject it; README claims gas compatibility)
**Impact:** Trailing junk after a by-element FMUL is silently dropped, so a mistyped extra register does not fail the assemble
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[0], v0.2s], u_bit=0, opcode=0b1001)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word)
**Severity:** medium
**Root cause:** neon.rs:1615 checks only `operands.len() < 3`, so length 4 is accepted and operands[3] is never read
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1615`
```rust
    if operands.len() < 3 { return Err("NEON float by-element requires 3 operands".to_string()); }
```
**Suggested fix:** Reject anything other than exactly 3 operands
```rust
    if operands.len() != 3 { return Err("NEON float by-element requires 3 operands".to_string()); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects fmul v0.2s, v0.2s, v0.s[0], v0.2s)
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, idx_raw = 0, shape = ("2s", "s", 3), insn = (0, 9, "fmul")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_extra_operand
