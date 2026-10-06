# Bug: encode_smaddl accepts W/X mixes that llvm-mc rejects
**Law:** SMADDL requires `Xd, Wn, Wm, Xa`. Any other W/X mix must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler encodes `smaddl w0, w0, w0, w0` (and other mixed-width forms) as if the registers were `x0, w0, w0, x0`, emitting a 64-bit SMADDL word for 32-bit sources/dest. Callers that pass W registers get the wrong instruction class.
**Function:** encode_smaddl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:653
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0")]`
**Expected:** `Err`
**Actual:** `Ok(Word(0x9b200000))` — width from get_reg is discarded
**Severity:** medium
**Root cause:** data_processing.rs:654-657 binds `let (rd, _) = get_reg(...)` (and the same for Rn/Rm/Ra), discarding the is_64 flag, then packs only the 5-bit register numbers.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Require Xd/Xa and Wn/Wm before packing.
```rust
    let (rd, rd64) = get_reg(operands, 0)?;
    let (rn, rn64) = get_reg(operands, 1)?;
    let (rm, rm64) = get_reg(operands, 2)?;
    let (ra, ra64) = get_reg(operands, 3)?;
    if !rd64 || rn64 || rm64 || !ra64 {
        return Err("smaddl requires Xd, Wn, Wm, Xa".into());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_wrong_width -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_smaddl_pbt::encode_smaddl_neg_wrong_width stdout ----
Test failed: SMADDL requires Xd, Wn, Wm, Xa; rd64=false rn64=false rm64=false ra64=false must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:427.
minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, rd64 = false, rn64 = false, rm64 = false, ra64 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
