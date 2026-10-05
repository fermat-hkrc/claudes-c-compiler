# Bug: encode_neon_eor3 accepts GPR, bare V, and non-arrangement operands
**Law:** EOR3 operands must be arranged V registers `Vn.16B`; GPR, FP, SP, bare V, and `Xn.16b` must be rejected
**Impact:** `eor3 x0, v0.16b, v0.16b, v0.16b` encodes as `eor3 v0.16b, v0.16b, v0.16b, v0.16b` because `parse_reg_num` maps `x0` to register 0, so a GPR dest is silently turned into v0
**Function:** encode_neon_eor3
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1112
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_eor3([Reg("x0"), v0.16b, v0.16b, v0.16b])
**Expected:** Err (llvm-mc/gas require Vd.16B)
**Actual:** Ok(Word(0xce000000)) — same encoding as `eor3 v0.16b, v0.16b, v0.16b, v0.16b`
**Severity:** medium
**Root cause:** encode_neon_eor3:1115 calls get_neon_reg, whose Operand::Reg arm (neon.rs:14-17) accepts any parse_reg_num name (x/w/d/s/q/v/h/b/sp) and returns an empty arrangement that EOR3 then ignores
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1115`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a V-prefixed RegArrangement with arrangement 16b
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if !matches!(&operands[0], Operand::RegArrangement { reg, .. } if reg.starts_with('v') || reg.starts_with('V'))
        || arr_d != "16b"
    {
        return Err("eor3 requires Vd.16b".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_eor3_pbt::encode_neon_eor3_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs:146:1:
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects eor3 x0, v0.16b, v0.16b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs:396.
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, kind = 0, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
