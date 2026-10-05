# Bug: encode_neon_faddp ignores source arrangements
**Law:** Vector FADDP Vd.T, Vn.T, Vm.T requires the same arrangement T on all three operands
**Impact:** Mismatched assembly such as `faddp v0.2d, v0.2s, v0.2s` is encoded as a .2d FADDP using the source register numbers, dropping Vn/Vm arrangements; gas/llvm-mc reject the same text
**Function:** encode_neon_faddp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1686
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_faddp([v0.2d, v0.2s, v0.2s])
**Expected:** Err (mismatched arrangement); llvm-mc rejects the same text
**Actual:** Ok(Word) encoding FADDP v0.2d, v0.2d, v0.2d (Q=1, sz=1 from dest T only)
**Severity:** medium
**Root cause:** neon.rs:1691-1692 bind source arrangements as `_` and derive Q/sz only from dest T, so Vn/Vm arrangements never participate in the encode
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1691`
```rust
        let (rn, _) = get_neon_reg(operands, 1)?;
        let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn and Vm arrangements to equal Vd.T
```rust
        let (rn, arr_n) = get_neon_reg(operands, 1)?;
        let (rm, arr_m) = get_neon_reg(operands, 2)?;
        if arr_n != arr_d || arr_m != arr_d {
            return Err(format!("faddp: mismatched arrangement: {arr_d} vs {arr_n} vs {arr_m}"));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp_neg_mismatch_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_faddp_pbt::encode_neon_faddp_neg_mismatch_t' (2472509) panicked at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:217:1:
Test failed: mismatched T must Err (llvm-mc rejects faddp v0.2d, v0.2s, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:392.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "2d", tn = "2s", tm = "2s"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
