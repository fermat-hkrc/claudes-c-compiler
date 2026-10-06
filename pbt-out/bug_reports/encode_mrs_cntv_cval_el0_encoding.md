# Bug: encode_mrs encodes CNTV_CVAL_EL0 with the wrong sysreg field
**Law:** encode_mrs([Reg(Xt), Symbol("cntv_cval_el0")]) must equal llvm-mc("mrs Xt, cntv_cval_el0"), which is ARM S3_3_C14_C3_2 (op2=2)
**Impact:** `mrs x0, cntv_cval_el0` reads S3_3_C14_C3_4 (op2=4) instead of the virtual timer compare-value register. Kernel/runtime code that samples CNTV_CVAL_EL0 through this assembler gets a reserved encoding and the wrong timer value.
**Function:** encode_mrs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:54
**Detected by:** Differential — llvm-mc AArch64 assembler (named sysreg word)
**Minimal input:** encode_mrs(&[Operand::Reg("x0".into()), Operand::Symbol("cntv_cval_el0".into())])  (assembly: `mrs x0, cntv_cval_el0`)
**Expected:** Ok(Word(0xd53be340))  // llvm-mc / ARM S3_3_C14_C3_2, encoding field 0xdf1a
**Actual:** Ok(Word(0xd53be380))  // encoding field 0xdf1c (op2=4)
**Severity:** high
**Root cause:** system.rs:125 `"cntv_cval_el0" => 0xdf1c` uses op2=4. ARM CNTV_CVAL_EL0 is op0=3, op1=3, CRn=14, CRm=3, op2=2 → 0xdf1a.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:125`
```rust
        "cntv_cval_el0" => 0xdf1c,
```
**Suggested fix:** Use the ARM encoding 0xdf1a.
```rust
        "cntv_cval_el0" => 0xdf1a,
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_cntv_cval_el0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_mrs_pbt::encode_mrs_diff_named_word' panicked at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:419:1:
Test failed: assertion failed: `(left == right)`
  left: `3577471872`,
 right: `3577471808`: SUT vs llvm-mc for mrs x0, cntv_cval_el0 at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:477.
minimal failing input: name = "cntv_cval_el0", xt = "x0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
