# Bug: encode_fp_arith_d treats a non-rounding-mode 4th operand as DYN
**Law:** The optional 4th operand of FADD.D/FSUB.D/FMUL.D/FDIV.D must be a rounding mode; any other 4th operand must return Err. llvm-mc rejects it ("operand must be a valid floating point rounding mode mnemonic").
**Impact:** `fadd.d f0, f0, f0, 0` still encodes as `fadd.d f0, f0, f0` with rm=DYN. A mistaken 4th token is silently reinterpreted as dynamic rounding.
**Function:** encode_fp_arith_d
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:77
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_arith_d([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 1)
**Expected:** Err
**Actual:** Ok(Word(33583187)) which is 0x02007053, the encoding of fadd.d f0, f0, f0 with rm=DYN
**Severity:** medium
**Root cause:** float.rs:69 (in callee encode_fp_arith, which encode_fp_arith_d forwards to unchanged) maps every non-RoundingMode 4th operand to rm=0b111 (dynamic) instead of returning Err, then line 74 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:69`
```rust
            _ => 0b111, // dynamic
```
**Suggested fix:** Return Err on a 4th operand that is not a RoundingMode.
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
cargo test --lib encode_fp_arith_d_pbt -- --test-threads=1
cargo test --lib test_encode_fp_arith_d_regression_non_rm_fourth -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th non-RoundingMode operand must Err for fadd.d f0, f0, f0 (optional rm only); got Ok(Word(33583187)) at src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs:523.
minimal failing input: (mn, f7) = (
    "fadd.d",
    1,
), rd = "f0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
