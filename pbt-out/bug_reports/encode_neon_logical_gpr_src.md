# Bug: encode_neon_logical encodes GPR/SP/bare/FP sources as NEON registers
**Law:** AND/ORR/EOR Vd.T, Vn.T, Vm.T require arranged vector registers; a GPR, SP, bare v, or scalar FP source must be rejected
**Impact:** `and v0.8b, x0, x0` is caller-reachable (encode_logical dispatches on dest RegArrangement) and silently encodes as `and v0.8b, v0.8b, v0.8b`, so a mixed GPR/vector typo produces the wrong instruction
**Function:** encode_neon_logical
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:297
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_logical([v0.8b, x0, x0], opc=0)
**Expected:** Err (llvm-mc/gas reject GPR sources on vector AND)
**Actual:** Ok(EncodeResult::Word) of `and v0.8b, v0.8b, v0.8b` (parse_reg_num maps x0→0)
**Severity:** medium
**Root cause:** neon.rs:299-300 call get_neon_reg, which accepts Operand::Reg and returns an empty arrangement; encode_neon_logical then packs the GPR number into Rn/Rm without requiring a non-empty arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:299`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require a non-empty arrangement on every operand (reject Operand::Reg)
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d.is_empty() || arr_n.is_empty() || arr_m.is_empty() {
        return Err("NEON logical requires arranged vector registers".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_gpr_src -- --test-threads=1
```
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / FP source must Err (llvm-mc rejects and v0.8b, x0, x0) at src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs:486.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", opc = 0, kind = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
