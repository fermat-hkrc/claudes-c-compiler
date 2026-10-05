# Bug: encode_cnt accepts GPR, SP, bare V, and FP scalar operands
**Law:** CNT operands must be arranged NEON registers Vd.<T>, Vn.<T> with T in {8b,16b}
**Impact:** `cnt x0, x0` (also `cnt sp, v0.8b`, `cnt v0, v1`, `cnt d0, d1`) encodes as CNT v0.8b, v0.8b (0x0e205800), mapping GPRs/SP/bare V/FP scalars onto SIMD register numbers instead of rejecting them
**Function:** encode_cnt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:23
**Detected by:** Negative/Error Contract
**Minimal input:** encode_cnt([Reg("x0"), Reg("x0")])
**Expected:** Err (llvm-mc: invalid operand; gas: operand 1 must be an SVE vector register / invalid use of vector register)
**Actual:** Ok(Word(0x0e205800))
**Severity:** high
**Root cause:** neon.rs:31-33 calls get_neon_reg, which accepts Operand::Reg via parse_reg_num (x/w/d/s/q/v/h/b/sp), then treats the empty arrangement as Q=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:31`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 }; // .8b -> Q=0, .16b -> Q=1
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} on both operands
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("cnt: unsupported arrangement .{arr_d}, expected .8b or .16b"));
    }
    if arr_n != arr_d {
        return Err(format!("cnt: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_cnt_pbt::encode_cnt_neg_gpr_bare_sp' panicked at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:166:1:
Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects cnt x0, x0) at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:351.
minimal failing input: rd = 0, rn = 0, t = "8b", kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
