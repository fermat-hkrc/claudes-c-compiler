# Bug: encode_msr encodes CNTV_CVAL_EL0 with the wrong sysreg field
**Law:** encode_msr([Symbol("cntv_cval_el0"), Reg(Xt)]) must equal llvm-mc("msr cntv_cval_el0, Xt"), which is ARM S3_3_C14_C3_2 (op2=2)
**Impact:** `msr cntv_cval_el0, x0` writes S3_3_C14_C3_4 (op2=4) instead of the virtual timer compare-value register. Kernel/runtime code that programs CNTV_CVAL_EL0 through this assembler hits a reserved encoding and the wrong timer register.
**Function:** encode_msr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:262
**Detected by:** Differential — llvm-mc AArch64 assembler (named sysreg word)
**Minimal input:** encode_msr(&[Operand::Symbol("cntv_cval_el0".into()), Operand::Reg("x0".into())])  (assembly: `msr cntv_cval_el0, x0`)
**Expected:** Ok(Word(0xd51be340))  // llvm-mc / ARM S3_3_C14_C3_2, encoding field 0xdf1a
**Actual:** Ok(Word(0xd51be380))  // encoding field 0xdf1c (op2=4)
**Severity:** high
**Root cause:** system.rs:364 `"cntv_cval_el0" => 0xdf1c` uses op2=4. ARM CNTV_CVAL_EL0 is op0=3, op1=3, CRn=14, CRm=3, op2=2 → 0xdf1a.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:364`
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
cargo test --lib test_encode_msr_regression_cntv_cval_el0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_msr_pbt::encode_msr_diff_named' panicked at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:444:1:
Test failed: SUT vs llvm-mc for msr cntv_cval_el0, x0: sut=d51be380 mc=d51be340 at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:455.
minimal failing input: name = "cntv_cval_el0", xt = "x0"
	successes: 78
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_msr_pbt.rs
