# Bug: encode_neon_cmp_zero accepts GPR/non-V names as NEON registers
**Law:** ∀ kind ∈ {arranged-x-prefix dest, GPR src, …}. llvm-mc rejects ∧ encode_neon_cmp_zero(ops, 0, 0b01001) = Err
**Impact:** `cmeq x0.8b, v0.8b, #0` and `cmeq v0.8b, x0, #0` assemble as if the GPR were v0. Invalid assembly becomes a silent v-register encoding.
**Function:** encode_neon_cmp_zero
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:189
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_cmp_zero([RegArrangement{reg:"x0", arrangement:"8b"}, v0.8b], u=0, opcode=0b01001)  // cmeq x0.8b, v0.8b, #0
**Expected:** Err
**Actual:** Ok(Word) identical to `cmeq v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** get_neon_reg (neon.rs:9-17) feeds any register name through parse_reg_num, which accepts x/w/d/s/q/v/h/b prefixes (mod.rs:207-209). encode_neon_cmp_zero does not require a V register.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:9`
```rust
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
```
**Suggested fix:** Require a V-prefixed RegArrangement on both operands; reject Operand::Reg (GPR/bare) for this helper.
```rust
    fn require_v_arr(operands: &[Operand], idx: usize) -> Result<(u32, String), String> {
        match operands.get(idx) {
            Some(Operand::RegArrangement { reg, arrangement })
                if reg.starts_with('v') || reg.starts_with('V') =>
            {
                get_neon_reg(operands, idx)
            }
            other => Err(format!("expected NEON Vn.T at operand {}, got {:?}", idx, other)),
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_gpr_bare_nonv -- --test-threads=1
```
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / non-V prefix must Err (llvm-mc rejects cmeq x0.8b, v0.8b, #0) at src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs:409.
minimal failing input: rd = 0, rn = 0, t = "8b", kind = 4
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
