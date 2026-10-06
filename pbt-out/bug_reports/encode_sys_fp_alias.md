# Bug: encode_sys rejects the GNU Xt alias fp
**Law:** If llvm-mc / GNU as accept `sys #<op1>, Cn, Cm, #<op2>, fp` as Xt=x29, then encode_sys must encode the same word
**Impact:** Any assembly that uses the standard `fp` alias for x29 (common in GCC-emitted and hand-written AArch64) fails to assemble `sys` instead of producing SYS with Rt=29.
**Function:** encode_sys
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:448
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_sys("#0, c0, c0, #0, fp")  (assembly: `sys #0, c0, c0, #0, fp`)
**Expected:** Ok(Word(0xd508001d))  // SYS #0, C0, C0, #0, x29
**Actual:** Err("sys: invalid register: fp")
**Severity:** medium
**Root cause:** system.rs:462-463 lowercases the Xt token and calls parse_reg_num, which has no `fp` arm (mod.rs:247 only maps sp/wsp/xzr/wzr/lr plus x/w/d/s/q/v/h/b prefixes), so a valid GNU alias is reported as an invalid register.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:462`
```rust
        let reg = parts[4].trim().to_lowercase();
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```
**Suggested fix:** Treat `fp` as x29 (in parse_reg_num, or in encode_sys before the lookup).
```rust
        let mut reg = parts[4].trim().to_lowercase();
        if reg == "fp" {
            reg = "x29".to_string();
        }
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_fp_alias -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_sys_pbt::encode_sys_diff_valid' panicked at src/backend/arm/assembler/encoder/encode_sys_pbt.rs:331:1:
Test failed: SUT vs llvm-mc for sys #0,c0,c0,#0,fp: sys: invalid register: fp.
minimal failing input: case = (
    0,
    0,
    0,
    0,
    29,
    "#0, c0, c0, #0, fp",
    "#0,c0,c0,#0,fp",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sys_pbt.rs
