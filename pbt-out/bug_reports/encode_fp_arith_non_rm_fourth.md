# Bug: encode_fp_arith treats a non-rounding-mode 4th operand as DYN
**Law:** The optional 4th operand of FADD/FSUB/FMUL/FDIV must be a rounding mode (rne/rtz/rdn/rup/rmm/dyn). Any other 4th operand must return Err. llvm-mc rejects `fadd.s fa0, fa1, fa2, x1` and unknown rm mnemonics; float.rs:65 documents the 4th operand as an optional rounding mode.
**Impact:** `fadd.s f0, f0, f0, 0` (or a stray register/memory/CSR operand) still encodes as `fadd.s f0, f0, f0` with rm=DYN (0b111). A mistaken 4th token is silently reinterpreted as dynamic rounding instead of being diagnosed.
**Function:** encode_fp_arith
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:61
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_arith([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 0)
**Expected:** Err
**Actual:** Ok(Word(28755)) which is 0x00007053, the encoding of fadd.s f0, f0, f0 with rm=DYN
**Severity:** medium
**Root cause:** float.rs:69 maps every non-RoundingMode 4th operand to rm=0b111 (dynamic) instead of returning Err, then line 74 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:69`
```rust
            _ => 0b111, // dynamic
```
**Suggested fix:** Reject a 4th operand that is not a rounding mode.
```rust
            other => {
                return Err(format!(
                    "fp arith: expected rounding mode, got {:?}",
                    other
                ))
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_arith_neg_non_rm_fourth -- --test-threads=1
cargo test --lib test_encode_fp_arith_regression_non_rm_fourth -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th non-RoundingMode operand must Err for fadd.s f0, f0, f0 (optional rm only); got Ok(Word(28755)) at src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs:501.
minimal failing input: (mn, f7) = (
    "fadd.s",
    0,
), rd = "f0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
