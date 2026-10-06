# Bug: encode_fma maps a non-RoundingMode 5th operand to rm=DYN
**Law:** The optional 5th operand of FMADD/FMSUB/FNMSUB/FNMADD must be a rounding-mode mnemonic; any other operand kind must return Err. llvm-mc rejects a 5th non-RM token ("operand must be a valid floating point rounding mode mnemonic"); encode_instruction passes the full operand slice through to encode_fma.
**Impact:** Malformed `fmadd.s f0, f0, f0, f0, 0` still assembles as `fmadd.s f0, f0, f0, f0` with dynamic rounding (rm=111). A mistyped 5th token is silently rewritten to DYN, so the assembler emits a valid R4-type FMA word instead of diagnosing the bad operand.
**Function:** encode_fma
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:175
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fma([Reg("f0"), Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 0b1000011, 0)
**Expected:** Err
**Actual:** Ok(Word(28739)) which is 0x00007043, the encoding of fmadd.s f0, f0, f0, f0 (rm=DYN)
**Severity:** medium
**Root cause:** float.rs:180-184 treats any non-RoundingMode 5th operand as rm=0b111 instead of returning Err.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:183`
```rust
            _ => 0b111,
```
**Suggested fix:** Return Err when the 5th operand is not a RoundingMode.
```rust
    let rm = if operands.len() > 4 {
        match &operands[4] {
            Operand::RoundingMode(s) => parse_rm(s),
            other => {
                return Err(format!("fma: expected rounding mode, got {:?}", other));
            }
        }
    } else {
        0b111
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fma_pbt -- --test-threads=1
cargo test --lib test_encode_fma_regression_non_rm_fifth -- --test-threads=1
```
**Raw output:**
```text
Test failed: 5th non-RoundingMode operand must Err for fmadd.s f0, f0, f0, f0 (optional rm only); got Ok(Word(28739)) at src/backend/riscv/assembler/encoder/encode_fma_pbt.rs:589.
minimal failing input: (mn, opc, fmt) = (
    "fmadd.s",
    67,
    0,
), rd = "f0", rs1 = "f0", rs2 = "f0", rs3 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
