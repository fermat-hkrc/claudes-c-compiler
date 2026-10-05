# Bug: encode_neon_ld_st_single accepts a non-consecutive register list
**Law:** ARM ISA requires the registers in an LD/ST single-structure list to be consecutive (wrapping); a gap must be Err.
**Impact:** `st2 {v0.b, v2.b}[0], [x0]` encodes as `{v0.b, v1.b}` (only regs[0] is used), so a non-sequential list silently stores the wrong second register.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({v0.b, v2.b}[0]), Mem{x0,0}], is_load=false, num_structs=2)
**Expected:** Err
**Actual:** Ok(Word) — only regs[0] supplies Rt; v2 is ignored
**Severity:** low (documented by the author)
**Root cause:** neon.rs:918 TODO admits the consecutive-register check is missing; the encoder reads only regs[0] for Rt and arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:918`
```rust
    // TODO: validate that registers in the list are consecutive (ARM ISA requirement)
```
**Suggested fix:** After the length check, require wrapping consecutiveness.
```rust
    for i in 1..regs.len() {
        let prev = parse_reg_num(reg_name(&regs[i - 1])).ok_or("invalid register in list")?;
        let cur = parse_reg_num(reg_name(&regs[i])).ok_or("invalid register in list")?;
        if cur != (prev + 1) % 32 {
            return Err("registers must be sequential".to_string());
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_nonconsecutive -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_nonconsecutive' panicked at src/backend/arm/assembler/encoder/neon.rs:13479:9:
st2 {v0.b, v2.b}[0], [x0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
