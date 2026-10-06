# Bug: encode_fcvt_int treats a non-rounding-mode 3rd operand as DYN
**Law:** The optional 3rd operand of FCVT.{W,WU,L,LU}.{S,D} must be a rounding mode; any other 3rd operand must return Err. llvm-mc rejects it ("operand must be a valid floating point rounding mode mnemonic").
**Impact:** `fcvt.w.s x0, f0, 0` still encodes as `fcvt.w.s x0, f0` with rm=DYN. A mistaken 3rd token is silently reinterpreted as dynamic rounding.
**Function:** encode_fcvt_int
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:116
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fcvt_int([Reg("x0"), Reg("f0"), Imm(0)], 0b1100000, 0)
**Expected:** Err
**Actual:** Ok(Word(3221254227)) which is 0xc0007053, the encoding of fcvt.w.s x0, f0 with rm=DYN
**Severity:** medium
**Root cause:** float.rs:123 maps every non-RoundingMode 3rd operand to rm=0b111 (dynamic) instead of returning Err, then line 128 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:123`
```rust
            _ => 0b111,
```
**Suggested fix:** Return Err on a 3rd operand that is not a RoundingMode.
```rust
            other => {
                return Err(format!(
                    "fcvt int: expected rounding mode, got {:?}",
                    other
                ))
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_int -- --test-threads=1
cargo test --lib test_encode_fcvt_int_regression_non_rm_third -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3rd non-RoundingMode operand must Err for fcvt.w.s x0, f0 (optional rm only); got Ok(Word(3221254227)) at src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:537.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.w.s",
    96,
    0,
), rd = "x0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
