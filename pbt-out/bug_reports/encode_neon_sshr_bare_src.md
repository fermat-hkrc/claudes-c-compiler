# Bug: encode_neon_sshr accepts a bare (non-arrangement) source register
**Law:** SSHR is `Vd.T, Vn.T, #shift`; a bare `v0` or a GPR such as `x0.8b` must be rejected
**Impact:** Invalid assembly `sshr v0.8b, v0, #1` is encoded as `sshr v0.8b, v0.8b, #1`. The same helper also accepts `sshr x0.8b, v0.8b, #1` as NEON SSHR with Rd taken from the X-register number, so the assembler emits a SIMD instruction the source text did not name
**Function:** encode_neon_sshr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1205
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sshr([v0.8b, Operand::Reg("v0"), #1])
**Expected:** Err (llvm-mc/gas require Vn.T)
**Actual:** Ok(Word) — encoded as `sshr v0.8b, v0.8b, #1`
**Severity:** medium
**Root cause:** neon.rs:1210 calls get_neon_reg, whose Operand::Reg arm returns an empty arrangement that the caller discards; parse_reg_num also accepts x/w prefixes, so a GPR with a fake arrangement is treated as Vd
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1210`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require RegArrangement with a V-prefixed register on dest and src
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n.is_empty() {
        return Err("sshr source must be Vn.T".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sshr_regression_bare_src -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sshr_pbt::encode_neon_sshr_neg_gpr_or_bare' (2343197) panicked at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:263:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects sshr v0.8b, v0, #1) at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:482.
minimal failing input: rd = 0, rn = 0, kind = 1, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
