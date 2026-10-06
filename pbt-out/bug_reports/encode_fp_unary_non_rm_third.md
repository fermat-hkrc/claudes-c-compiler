# Bug: encode_fp_unary treats a non-rounding-mode 3rd operand as DYN
**Law:** The optional 3rd operand of FSQRT.S/FSQRT.D must be a rounding mode; any other 3rd operand must return Err. llvm-mc rejects it ("operand must be a valid floating point rounding mode mnemonic").
**Impact:** `fsqrt.s f0, f0, 0` still encodes as `fsqrt.s f0, f0` with rm=DYN. A mistaken 3rd token is silently reinterpreted as dynamic rounding.
**Function:** encode_fp_unary
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:81
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_unary([Reg("f0"), Reg("f0"), Imm(0)], 0b0101100, 0)
**Expected:** Err
**Actual:** Ok(Word(1476423763)) which is 0x58007053, the encoding of fsqrt.s f0, f0 with rm=DYN
**Severity:** medium
**Root cause:** float.rs:87 maps every non-RoundingMode 3rd operand to rm=0b111 (dynamic) instead of returning Err, then line 92 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:87`
```rust
            _ => 0b111,
```
**Suggested fix:** Return Err on a 3rd operand that is not a RoundingMode.
```rust
            other => {
                return Err(format!(
                    "fp unary: expected rounding mode, got {:?}",
                    other
                ))
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_unary_pbt -- --test-threads=1
cargo test --lib test_encode_fp_unary_regression_non_rm_third -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3rd non-RoundingMode operand must Err for fsqrt.s f0, f0 (optional rm only); got Ok(Word(1476423763)) at src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:510.
minimal failing input: (mn, f7) = (
    "fsqrt.s",
    44,
), rd = "f0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
