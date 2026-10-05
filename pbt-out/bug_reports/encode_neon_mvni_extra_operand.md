# Bug: encode_neon_mvni ignores extra operands
**Law:** When llvm-mc/gas reject a third non-shift operand, encode_neon_mvni must return Err
**Impact:** Invalid assembly such as `mvni v0.4h, #0, v0.4h` is assembled as a two-operand MVNI, hiding typos
**Function:** encode_neon_mvni
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1333
**Detected by:** Negative/error contract vs llvm-mc
**Minimal input:** encode_neon_mvni([v0.4h, #0, v0.4h])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) encoding `mvni v0.4h, #0`
**Severity:** medium
**Root cause:** neon.rs:1334 only rejects operands.len() < 2; a third non-Shift token is ignored (2s/4s treats non-Shift as no-shift; 4h/8h never inspects operands[2])
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1334`
```rust
    if operands.len() < 2 {
        return Err("mvni requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject len > 3, and require that operand 2 (when present) is a legal Shift; reject a third non-shift token
```rust
    if operands.len() < 2 {
        return Err("mvni requires 2 operands".to_string());
    }
    if operands.len() > 3 {
        return Err("mvni: extra operand".to_string());
    }
    if operands.len() == 3 && !matches!(operands.get(2), Some(Operand::Shift { .. })) {
        return Err("mvni: expected optional shift".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni_neg_extra_and_illegal_shift -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mvni_pbt::encode_neon_mvni_neg_extra_and_illegal_shift' panicked at src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs:278:1:
Test failed: extra/illegal shift must Err (llvm-mc rejects mvni v0.4h, #0, v0.4h)
minimal failing input: rd = 0, extra = 0, t = "4h", imm8 = 0, kind = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs (test_encode_neon_mvni_regression_extra_operand)
