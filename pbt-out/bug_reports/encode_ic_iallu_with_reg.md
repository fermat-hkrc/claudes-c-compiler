# Bug: encode_ic accepts a register on IALLUIS/IALLU
**Law:** If llvm-mc / GNU as reject `ic ialluis, Xt` / `ic iallu, Xt`, then encode_ic("ialluis, Xt") / encode_ic("iallu, Xt") must return Err
**Impact:** `ic ialluis, x0` assembles as SYS with Rt=0 instead of being rejected. IALLUIS/IALLU have no Xt in ARM syntax; a stray register is silently patched into bits[4:0], so a typo is not diagnosed.
**Function:** encode_ic
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:401
**Detected by:** Negative/Error Contract (extra Xt on IALLUIS/IALLU; llvm-mc rejects)
**Minimal input:** encode_ic("ialluis, x0")  (assembly: `ic ialluis, x0`)
**Expected:** Err (llvm-mc: "specified ic op does not use a register"; gas: "extraneous register at operand 2")
**Actual:** Ok(Word(0xd5087100))  // IALLUIS with Rt=x0 instead of XZR
**Severity:** medium
**Root cause:** system.rs:416 always writes `(base & !0x1F) | rt` after parsing an optional register; IALLUIS/IALLU never check that a second operand is forbidden.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:416`
```rust
    let word = (base & !0x1F) | rt;
```
**Suggested fix:** Reject a register operand for IALLUIS and IALLU before encoding.
```rust
    if matches!(op_name.as_str(), "ialluis" | "iallu") && parts.len() > 1 {
        return Err(format!("ic: {} does not take a register", op_name));
    }
    let word = (base & !0x1F) | rt;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ic_regression_ialluis_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ic_pbt::encode_ic_neg_iallu_with_reg' (2564597) panicked at src/backend/arm/assembler/encoder/encode_ic_pbt.rs:345:1:
Test failed: IALLU* with register must Err (llvm-mc rejects ic ialluis, x0); SUT raw "ialluis, x0": Word(3574100224).
minimal failing input: op = "ialluis", xt = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ic_pbt.rs
