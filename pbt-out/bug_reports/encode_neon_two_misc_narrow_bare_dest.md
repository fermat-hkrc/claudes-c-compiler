# Bug: encode_neon_two_misc_narrow encodes a bare V dest as XTN Vd.8B
**Law:** XTN dest must be a NEON V register with arrangement (Vd.Tb); gas/llvm-mc reject `xtn v0, v0.8h` and `xtn x0, v0.8h`
**Impact:** A dest without arrangement (or with a GPR prefix) is encoded as the corresponding V register, so a mistyped `xtn v0, v0.8h` silently becomes `xtn v0.8b, v0.8h`. encode() passes operands through, so the witness is caller-reachable.
**Function:** encode_neon_two_misc_narrow
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:206
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_two_misc_narrow([Reg("v0"), v0.8h], u_bit=0, opcode=0b10010, is_high=false)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0x0e212800)) — get_neon_reg accepts Operand::Reg; dest arrangement is unused so a bare v0 is encoded as XTN Q=0 size=00
**Severity:** medium
**Root cause:** neon.rs:210 calls get_neon_reg which accepts Operand::Reg, then discards dest arrangement; encode_neon_two_misc_narrow never requires a V-prefixed arrangement dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:210`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject a destination that is not a V-prefixed RegArrangement
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    match &operands[0] {
        Operand::RegArrangement { reg, .. } if reg.to_ascii_lowercase().starts_with('v') => {}
        _ => return Err("narrow: destination must be a V register with arrangement".to_string()),
    }
    let _ = arr_d;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow_neg_gpr_or_bare -- --test-threads=1
cargo test --lib test_encode_neon_two_misc_narrow_regression_bare_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_narrow_pbt::encode_neon_two_misc_narrow_neg_gpr_or_bare' (2513879) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:481:1:
Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects xtn v0, v0.8h) at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:536.
minimal failing input: rd = 0, rn = 0, kind = 2, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
