# Bug: encode_neon_ld_st_multi ignores non-consecutive registers and encodes from the first only
**Law:** ARM AdvSIMD multiple-structure lists must be sequential (wrapping v0–v31); a non-consecutive list must Err
**Impact:** `st2 {v0.8b, v2.8b}, [x0]` is encoded as `st2 {v0.8b, v1.8b}, [x0]` (Rt=v0). The second listed register is dropped, so the wrong pair is stored.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_multi([RegList({v0.8b, v2.8b}), Mem{x0,0}], is_load=false, num_structs=2)
**Expected:** Err (llvm-mc: registers must be sequential)
**Actual:** Ok(Word(0x0c008000)) — encoded as st2 from v0 (implies v1)
**Severity:** high
**Root cause:** neon.rs:1016-1023 reads only regs[0] for Rt and arrangement and uses regs.len() for the count; later registers are never checked for consecutiveness.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1016`
```rust
            let (first_reg, arrangement) = match &regs[0] {
                Operand::RegArrangement { reg, arrangement } => {
                    (parse_reg_num(reg).ok_or("invalid reg")?, arrangement.clone())
                }
                _ => return Err(format!("ld{}/st{}: expected RegArrangement in list", num_structs, num_structs)),
            };
            (first_reg, arrangement, regs.len() as u32)
```
**Suggested fix:** After taking the first register, require each subsequent list entry to be (first+i) mod 32 with the same arrangement.
```rust
            for (i, r) in regs.iter().enumerate().skip(1) {
                match r {
                    Operand::RegArrangement { reg, arrangement: a } => {
                        let num = parse_reg_num(reg).ok_or("invalid reg")?;
                        if num != (first_reg + i as u32) % 32 || a != &arrangement {
                            return Err("ld/st multi: registers must be sequential".into());
                        }
                    }
                    _ => return Err("ld/st multi: expected RegArrangement in list".into()),
                }
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_nonconsecutive -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_nonconsecutive' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:731:5:
st2 {v0.8b, v2.8b}, [x0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
