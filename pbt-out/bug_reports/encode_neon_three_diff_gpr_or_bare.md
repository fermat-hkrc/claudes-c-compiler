# Bug: encode_neon_three_diff accepts a GPR as Vm
**Law:** Three-different NEON operands must be V registers with arrangements. encode_neon_three_diff([Vd.Ta, Vn.Tb, xN], …) must Err; llvm-mc rejects `saddl v0.8h, v0.8b, x0`
**Impact:** A general-purpose register in the Rm slot is encoded as Vn-relative SIMD Rm via parse_reg_num's x/w prefix, producing a well-formed ASIMDDIFF word for an instruction GNU gas rejects. Dest GPR and bare V dest have the same hole (arrangement discarded).
**Function:** encode_neon_three_diff
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:89
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_three_diff([v0.8h, v0.8b, Operand::Reg("x0")], u_bit=0, opcode=0, is_high=false)
**Expected:** Err
**Actual:** Ok(Word) with Rm=0
**Severity:** medium
**Root cause:** neon.rs:95 calls get_neon_reg, which accepts Operand::Reg; parse_reg_num maps `x0` to 0. The Rm arrangement is discarded (`_arr_m`), so a GPR looks like Vm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:95`
```rust
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Operand::RegArrangement with a `v` prefix on every operand; reject Operand::Reg and x/w/d/s/q/h/b names.
```rust
    fn require_v_arr(operands: &[Operand], idx: usize) -> Result<(u32, String), String> {
        match operands.get(idx) {
            Some(Operand::RegArrangement { reg, arrangement }) if reg.to_ascii_lowercase().starts_with('v') => {
                let num = parse_reg_num(reg).ok_or_else(|| format!("invalid NEON register: {}", reg))?;
                Ok((num, arrangement.clone()))
            }
            other => Err(format!("expected NEON Vn.T at operand {}, got {:?}", idx, other)),
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_gpr_or_bare -- --test-threads=1
```
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects saddl v0.8h, v0.8b, x0)
minimal failing input: rd = 0, rn = 0, rm = 0, insn = (0, 0, "saddl"), kind = 2, fp_prefix = "x"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
