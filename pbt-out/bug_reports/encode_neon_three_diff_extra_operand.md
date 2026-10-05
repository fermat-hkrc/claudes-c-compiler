# Bug: encode_neon_three_diff ignores a fourth operand
**Law:** encode_neon_three_diff requires exactly 3 operands; a fourth operand must return Err, matching llvm-mc/gas which reject `saddl Vd.Ta, Vn.Tb, Vm.Tb, Vextra`
**Impact:** Trailing garbage in a three-different instruction is assembled as if it were a valid 3-operand form, so the assembler accepts input GNU gas rejects.
**Function:** encode_neon_three_diff
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:89
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_three_diff([v0.8h, v0.8b, v0.8b, v0.8b], u_bit=0, opcode=0, is_high=false)
**Expected:** Err
**Actual:** Ok(Word) — the extra operand is never read
**Severity:** medium
**Root cause:** neon.rs:90 checks `operands.len() < 3` only, so length 4 is treated as success.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:90`
```rust
    if operands.len() < 3 {
        return Err("NEON three-different requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject any arity other than 3.
```rust
    if operands.len() != 3 {
        return Err("NEON three-different requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4 operands must Err (llvm-mc rejects saddl v0.8h, v0.8b, v0.8b, v0.8b)
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, tb = "8b", insn = (0, 0, "saddl")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
