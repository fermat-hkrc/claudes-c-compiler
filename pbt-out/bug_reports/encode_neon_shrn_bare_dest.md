# Bug: encode_neon_shrn accepts a bare V dest without arrangement
**Law:** Vector SHRN dest must be a NEON register with arrangement Vd.Tb. A bare `vN`, GPR `xN`/`wN`, or scalar prefix must be rejected.
**Impact:** `shrn v0, v0.8h, #1` is invalid AArch64 (llvm-mc: invalid operand) but the helper encodes it as `shrn v0.8b, v0.8h, #1` (dest arrangement discarded; `get_neon_reg` accepts `Operand::Reg`). Invalid assembly is silently accepted.
**Function:** encode_neon_shrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1436
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shrn([Reg("v0"), v0.8h, #1], opcode=0b100001, is_high=false)
**Expected:** Err (llvm-mc/gas require Vd.Tb)
**Actual:** Ok(Word) — same encoding as `shrn v0.8b, v0.8h, #1`
**Severity:** medium
**Root cause:** get_neon_reg (neon.rs:14-17) accepts Operand::Reg with an empty arrangement, and encode_neon_shrn (neon.rs:1438) discards dest arrangement, so a bare V dest is encoded as a vector register
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement at the dest (and source) slots in encode_neon_shrn; do not accept a bare Operand::Reg
```rust
    let (rd, arr_d) = match &operands[0] {
        Operand::RegArrangement { reg, arrangement } => {
            (parse_reg_num(reg).ok_or_else(|| format!("invalid NEON register: {}", reg))?, arrangement.clone())
        }
        other => return Err(format!("expected NEON register with arrangement at operand 0, got {:?}", other)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_bare_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_gpr_or_bare' (2365156) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:492:1:
Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects shrn v0, v0.8h, #1) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:552.
minimal failing input: rd = 0, rn = 0, kind = 2, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
