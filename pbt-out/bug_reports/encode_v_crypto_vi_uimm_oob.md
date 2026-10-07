# Bug: encode_v_crypto_vi truncates out-of-range uimm5
**Law:** ∀ vd, vs2 ∈ {0..31}, uimm ∉ {0..31}, funct6 ∈ {101011,100001}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) is Err
**Impact:** Immediates outside the documented 5-bit unsigned field wrap with `& 0x1F` (32 encodes as 0, -1 encodes as 31). Callers that pass an out-of-range round number get a silently wrong encoding instead of an assembler error.
**Function:** encode_v_crypto_vi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:191
**Detected by:** Negative/Error Contract — uimm5 out of range
**Minimal input:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(-1)], 0b101011)
**Expected:** Err (uimm5 domain is [0, 31])
**Actual:** Ok(Word(0xae0f8077)) — same encoding as `vsm3c.vi v0, v0, 31`
**Severity:** medium
**Root cause:** vector.rs:194 `get_imm(...) as u32 & 0x1F` masks the immediate and never range-checks it against [0, 31].
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:194`
```rust
    let uimm5 = get_imm(operands, 2)? as u32 & 0x1F;
```
**Suggested fix:** Reject immediates outside 0..=31 before packing.
```rust
    let imm = get_imm(operands, 2)?;
    if !(0..=31).contains(&imm) {
        return Err(format!("uimm5 out of range: {imm}"));
    }
    let uimm5 = imm as u32;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_uimm_oob_m1 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vi_pbt::encode_v_crypto_vi_neg_uimm_oob' (2891633) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:229:1:
Test failed: vsm3c.vi v0, v0, -1 must Err (uimm5 domain [0, 31]); got Ok(Word(2920259703)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:402.
minimal failing input: vd = 0, vs2 = 0, uimm = -1, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
