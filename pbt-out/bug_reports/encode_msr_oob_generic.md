# Bug: encode_msr masks out-of-range generic S-form fields instead of rejecting
**Law:** If llvm-mc / GNU as reject `msr s<op0>_<op1>_c<CRn>_c<CRm>_<op2>, Xt` with op0>3 (or op1>7, CRn/CRm>15, op2>7), then encode_msr must return Err
**Impact:** `msr s4_0_c1_c0_1, x0` encodes as `msr s0_0_c1_c0_1, x0` (op0 masked with `& 3`). An illegal sysreg name silently becomes a different, legal one.
**Function:** encode_msr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:262
**Detected by:** Negative/Error Contract (oob generic S-form; llvm-mc rejects op0=4)
**Minimal input:** encode_msr(&[Operand::Symbol("s4_0_c1_c0_1".into()), Operand::Reg("x0".into())])  (assembly: `msr s4_0_c1_c0_1, x0`)
**Expected:** Err (llvm-mc: "expected writable system register or pstate")
**Actual:** Ok(Word)  // op0=4 masked to op0=0
**Severity:** medium
**Root cause:** system.rs:381 `_ => parse_generic_sysreg(&sysreg)?` accepts the name, and sysreg_encoding masks each field (`op0 & 3`, `op1 & 7`, `crn & 0xF`, …) instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:381`
```rust
        _ => parse_generic_sysreg(&sysreg)?,
```
**Suggested fix:** Reject out-of-range generic fields before masking, or range-check in encode_msr after parse_generic_sysreg.
```rust
        _ => {
            let enc = parse_generic_sysreg(&sysreg)?;
            // reject if the textual fields did not fit ARM widths
            enc
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_oob_generic_s4 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_msr_pbt::encode_msr_neg_wrong_src_unknown_arity_oob' panicked at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:444:1:
Test failed: oob generic sysreg must Err (llvm-mc rejects msr s4_0_c1_c0_1, x0) at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:661.
minimal failing input: kind = 4, dest = "w0", name = "sp_el0", unknown = "foo", oob_g = "s4_0_c1_c0_1", oob_n = "dbgbcr16_el1", field = "daifset", imm = -2, xt = "x0"
	successes: 2
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_msr_pbt.rs
