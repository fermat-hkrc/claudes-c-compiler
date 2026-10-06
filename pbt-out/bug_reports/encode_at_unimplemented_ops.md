# Bug: encode_at rejects ARM AT ops S1E2/S1E3/S12E*
**Law:** If llvm-mc / GNU as accept `at s1e2r, x0` (and the other default-CPU AT ops), then encode_at([], "s1e2r, x0") must encode the same word
**Impact:** Valid address-translate instructions at EL2/EL3 and stage-1+2 (`at s1e2r`, `at s1e3w`, `at s12e1r`, …) fail to assemble. Kernel/hypervisor assembly that gas accepts is rejected.
**Function:** encode_at
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:426
**Detected by:** Differential (llvm-mc AArch64 assembler)
**Minimal input:** encode_at(&[], "s1e2r, x0")  (assembly: `at s1e2r, x0`)
**Expected:** Ok(Word(0xd50c7800))  // SYS #4, C7, C8, #0, X0
**Actual:** Err("unsupported at operation: s1e2r")
**Severity:** low (documented by the author)
**Documentation conflict:** encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen." admits a subset on an input the assembler API accepts (README.md:12 gas-compat). The comment documents the limitation rather than declaring S1E2/S1E3/S12E* invalid.
**Root cause:** system.rs:436-441 match table only lists S1E1R/S1E1W/S1E0R/S1E0W; every other ARM AT op hits the `_` arm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:441`
```rust
        _ => return Err(format!("unsupported at operation: {}", op_name)),
```
**Suggested fix:** Encode the remaining default-CPU AT ops as SYS with CRn=7, CRm=8.
```rust
        "s1e1r" => 0xd5087800u32,
        "s1e1w" => 0xd5087820,
        "s1e0r" => 0xd5087840,
        "s1e0w" => 0xd5087860,
        "s1e2r" => 0xd50c7800,
        "s1e2w" => 0xd50c7820,
        "s1e3r" => 0xd50e7800,
        "s1e3w" => 0xd50e7820,
        "s12e1r" => 0xd50c7880,
        "s12e1w" => 0xd50c78a0,
        "s12e0r" => 0xd50c78c0,
        "s12e0w" => 0xd50c78e0,
        _ => return Err(format!("unsupported at operation: {}", op_name)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_at_regression_s1e2r -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_at_pbt::encode_at_diff_arm_ops' (2586973) panicked at src/backend/arm/assembler/encoder/encode_at_pbt.rs:377:1:
Test failed: ARM AT op "s1e2r" must encode (llvm-mc accepts at s1e2r, x0): "unsupported at operation: s1e2r".
minimal failing input: op = "s1e2r", t = 0
	successes: 1
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_at_pbt.rs
