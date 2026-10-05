# Bug: encode_neon_bsl accepts GPR, SP, bare V, and FP as BSL operands
**Law:** BSL operands must be arranged NEON registers Vd.T, Vn.T, Vm.T; GPR/SP/bare-V/FP must be rejected
**Impact:** `bsl x0, x0, x0` encodes as `bsl v0.8b, v0.8b, v0.8b` (0x2e601c00). SP maps to v31 via parse_reg_num; bare `v0` and scalar `d0`/`s0`/`q0` likewise become Q=0 BSL. Invalid assembly becomes a plausible NEON instruction
**Function:** encode_neon_bsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:735
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bsl([x0, x0, x0])
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0x2e601c00)) — same as `bsl v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement; encode_neon_bsl then treats any non-"16b" arrangement as Q=0. parse_reg_num maps sp to 31 and accepts x/w/d/s/q/v prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} in encode_neon_bsl (empty arrangement from Operand::Reg then Errs)
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("bsl: unsupported arrangement: {}", arr_d));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_gpr_bare_sp' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects bsl x0, x0, x0) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:373.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
