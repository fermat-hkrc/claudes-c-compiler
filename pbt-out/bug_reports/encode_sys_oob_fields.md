# Bug: encode_sys masks out-of-range op1/CRn/CRm/op2 instead of rejecting
**Law:** If llvm-mc / GNU as reject SYS fields outside op1,op2 ∈ [0,7] and Cn,Cm ∈ C0–C15, then encode_sys must return Err
**Impact:** `sys #8, c0, c0, #0, x0` encodes as SYS #0 (8 & 7 = 0). An out-of-range immediate silently wraps and emits a different instruction. The same wrap applies to CRn/CRm ≥ 16 and op2 ≥ 8.
**Function:** encode_sys
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:448
**Detected by:** Negative/Error Contract (out-of-range SYS fields; llvm-mc rejects)
**Minimal input:** encode_sys("#8, c0, c0, #0, x0")  (assembly: `sys #8, c0, c0, #0, x0`)
**Expected:** Err (llvm-mc: immediate must be an integer in range [0, 7]; gas: immediate value out of range 0 to 7)
**Actual:** Ok(Word(0xd5080000))  // SYS #0, C0, C0, #0, x0
**Severity:** high
**Root cause:** system.rs:466 packs fields with (op1 & 7), (crn & 0xF), (crm & 0xF), (op2 & 7) after a successful u32 parse, so values that fit in u32 but not in the ARM field widths are truncated rather than rejected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:466`
```rust
    let word = 0xd5080000 | ((op1 & 7) << 16) | ((crn & 0xF) << 12) | ((crm & 0xF) << 8) | ((op2 & 7) << 5) | rt;
```
**Suggested fix:** Reject out-of-range fields before packing; keep the mask only as a debug assertion.
```rust
    if op1 > 7 || crn > 15 || crm > 15 || op2 > 7 {
        return Err(format!("sys: field out of range in {}", raw_operands));
    }
    let word = 0xd5080000 | (op1 << 16) | (crn << 12) | (crm << 8) | (op2 << 5) | rt;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_oob_op1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_sys_pbt::encode_sys_neg_oob_fields' panicked at src/backend/arm/assembler/encoder/encode_sys_pbt.rs:331:1:
Test failed: SYS with out-of-range op1/Cn/Cm/op2 must Err (llvm-mc rejects sys #8, c0, c0, #0, x0); SUT raw "#8, c0, c0, #0, x0": Word(3574071296).
minimal failing input: raw = "#8, c0, c0, #0, x0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sys_pbt.rs
