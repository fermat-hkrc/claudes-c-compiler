# Bug: encode_mrs masks out-of-range generic sysreg fields
**Law:** If llvm-mc rejects `mrs Xt, s<op0>_<op1>_c<CRn>_c<CRm>_<op2>` because a field is outside ARM widths (op0 0..=3, op1 0..=7, CRn/CRm 0..=15, op2 0..=7), then encode_mrs must return Err rather than encoding a different register.
**Impact:** `mrs x0, s4_0_c1_c0_1` is accepted and encoded as `s0_0_c1_c0_1` (op0 masked with `& 3`). An out-of-range S-form silently becomes a different system register.
**Function:** encode_mrs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:54
**Detected by:** Negative/Error Contract (oob generic S-form; llvm-mc rejects)
**Minimal input:** encode_mrs(&[Operand::Reg("x0".into()), Operand::Symbol("s4_0_c1_c0_1".into())])  (assembly: `mrs x0, s4_0_c1_c0_1`)
**Expected:** Err (llvm-mc: "expected readable system register")
**Actual:** Ok(Word)  // sysreg_encoding masks op0=4 to 0
**Severity:** medium
**Root cause:** parse_generic_sysreg parses the five fields as unbounded u32 and calls sysreg_encoding, which masks `op0 & 3`, `op1 & 7`, `CRn & 0xF`, `CRm & 0xF`, `op2 & 7` instead of rejecting out-of-range values. encode_mrs reaches this path for any name not in the named table.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:185`
```rust
    ((op0 & 3) << 14) | ((op1 & 7) << 11) | ((crn & 0xF) << 7) | ((crm & 0xF) << 3) | (op2 & 7)
```
**Suggested fix:** Range-check fields in parse_generic_sysreg before calling sysreg_encoding.
```rust
        if op0 > 3 || op1 > 7 || crn > 15 || crm > 15 || op2 > 7 {
            return Err(format!("unsupported system register: {}", name));
        }
        let enc = sysreg_encoding(op0, op1, crn, crm, op2);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_oob_generic_s4 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_mrs_pbt::encode_mrs_neg_unknown_arity_oob' panicked at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:418:1:
Test failed: oob generic sysreg must Err (llvm-mc rejects mrs x0, s4_0_c1_c0_1) at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:668.
minimal failing input: kind = 3, unknown = "foo", oob_g = "s4_0_c1_c0_1", oob_n = "dbgbcr16_el1", xt = "x0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
