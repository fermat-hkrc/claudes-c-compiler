# Bug: encode_neon_three_diff ignores Vm arrangement
**Law:** LONG three-different instructions require matching source arrangements: Vm.Tb = Vn.Tb. encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tm], …) must Err when Tm ≠ Tb, matching llvm-mc
**Impact:** `saddl v0.8h, v0.8b, v0.16b` is encoded as `saddl v0.8h, v0.8b, v0.8b` (Rm number kept, arrangement ignored). Mismatched sources assemble instead of being rejected.
**Function:** encode_neon_three_diff
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:89
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_three_diff([v0.8h, v0.8b, v0.16b], u_bit=0, opcode=0, is_high=false)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) — `_arr_m` is unused; size/Q come only from Vn
**Severity:** medium
**Root cause:** neon.rs:95 binds Rm's arrangement to `_arr_m` and never compares it to Vn.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:95`
```rust
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vm arrangement to equal Vn arrangement for LONG forms.
```rust
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_m != arr_n {
        return Err(format!("source arrangements must match: Vn.{} vs Vm.{}", arr_n, arr_m));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_rm_tb_mismatch -- --test-threads=1
```
**Raw output:**
```text
Test failed: Rm Tb must match Vn Tb; llvm-mc rejects saddl v0.8h, v0.8b, v0.16b
minimal failing input: rd = 0, rn = 0, rm = 0, tb = "8b", tm = "16b", insn = (0, 0, "saddl")
cc 95f2bdc5b5994d8ac8fdc350e8ab3c55b3b597eef5f9fd1d11969f806ce28098
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
