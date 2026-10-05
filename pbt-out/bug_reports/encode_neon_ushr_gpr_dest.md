# Bug: encode_neon_ushr accepts a GPR destination as a NEON register
**Law:** USHR is `Vd.T, Vn.T, #shift`; a GPR such as `x0.8b` must be rejected
**Impact:** Invalid assembly `ushr x0.8b, v0.8b, #1` is encoded as NEON USHR with Rd taken from the X-register number, so the assembler emits a SIMD instruction the source text did not name
**Function:** encode_neon_ushr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1179
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ushr([x0.8b, v0.8b, #1])
**Expected:** Err (llvm-mc/gas require Vd.T)
**Actual:** Ok(Word) — encoded as `ushr v0.8b, v0.8b, #1`
**Severity:** medium
**Root cause:** neon.rs:1183 calls get_neon_reg, which accepts any parse_reg_num prefix including x/w, so a GPR with a fake arrangement is treated as Vd
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1183`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a V-prefixed register on dest and src
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if !operands_is_v_reg(operands, 0) {
        return Err("ushr destination must be Vd.T".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ushr_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ushr_pbt::encode_neon_ushr_neg_gpr_or_bare' (2335775) panicked at src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:263:1:
Test failed: GPR/bare/non-arrangement kind=3 must Err (llvm-mc rejects ushr x0.8b, v0.8b, #1) at src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:479.
minimal failing input: rd = 0, rn = 0, kind = 3, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
