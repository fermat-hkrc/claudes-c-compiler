# Bug: encode_fcvt_from_int treats a non-rounding-mode 3rd operand as DYN
**Law:** The optional 3rd operand of FCVT.{S,D}.{W,WU,L,LU} must be a rounding mode; any other 3rd operand must return Err. llvm-mc rejects it ("operand must be a valid floating point rounding mode mnemonic").
**Impact:** `fcvt.s.w f0, x0, 0` still encodes as `fcvt.s.w f0, x0` with rm=DYN. A mistaken 3rd token is silently reinterpreted as dynamic rounding.
**Function:** encode_fcvt_from_int
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:131
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fcvt_from_int([Reg("f0"), Reg("x0"), Imm(0)], 0b1101000, 0)
**Expected:** Err
**Actual:** Ok(Word(3489689683)) which is 0xd0007053, the encoding of fcvt.s.w f0, x0 with rm=DYN
**Severity:** medium
**Root cause:** float.rs:137 maps every non-RoundingMode 3rd operand to rm=0b111 (dynamic) instead of returning Err, then line 142 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:137`
```rust
            _ => 0b111,
```
**Suggested fix:** Return Err on a 3rd operand that is not a RoundingMode.
```rust
            other => {
                return Err(format!(
                    "fcvt from int: expected rounding mode, got {:?}",
                    other
                ))
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_from_int -- --test-threads=1
cargo test --lib test_encode_fcvt_from_int_regression_non_rm_third -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3rd non-RoundingMode operand must Err for fcvt.s.w f0, x0 (optional rm only); got Ok(Word(3489689683)) at src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs:559.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.s.w",
    104,
    0,
), rd = "f0", rs1 = "x0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
