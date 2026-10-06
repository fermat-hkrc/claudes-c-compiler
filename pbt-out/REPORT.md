# PBT Campaign Report: encode_mov

## Summary

**Verdict:** 9 bugs (3 high, 6 medium/high): the worst is encode_mov treating WSP as WZR so `mov w0, wsp` becomes `mov w0, wzr`, plus silent extra-operand drop, mixed-width/FP-as-GPR, `mov sp, #imm` as MOVZ XZR, NEON lane wrap, arrangement mismatch, 4s Q=0, and 64-bit imm truncation on W.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mov
**Tests:** 15
**Result:** 5 passing, 10 failing properties / 10 bug reports (9 unique defects; B1 and B10 are the WSP miss seen by differential and invariant)
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_mov NOT LINKED). Rust cargo tests executed the production symbol. Sweep: manual arm audit + encode_mov_diff_alt_spellings.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mov | 15 | 9 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_mov treats WSP as WZR

**Formal:** ∀ rd,rm ∈ 0..31, is_64 ∈ {0,1}, rd_sp,rm_sp ∈ {0,1}. encode_mov([Reg(gpr(is_64,rd,rd_sp)), Reg(gpr(is_64,rm,rm_sp))]) = Word(llvm-mc("mov Rd, Rm"))
**Contract evidence:** documented data_processing.rs:129 "Check for MOV to/from SP: uses ADD Xd, Xn, #0"; README.md:293 "`mov` to/from `sp` encodes as `add Xd, Xn, #0`"
**Documentation conflict:** data_processing.rs:129 names `sp` only — it does not declare `wsp` invalid; gas/llvm-mc accept `mov w0, wsp` as ADD. The comment is incomplete, not an exclusion.
**Severity:** high
**Counterexample:** encode_mov([Reg("w0"), Reg("wsp")]) then compare to llvm-mc `mov w0, wsp`
**Expected / Actual:** Word(0x110003e0) ADD W0, WSP, #0 / Word(0x2a1f03e0) ORR W0, WZR, WZR
**Impact:** 32-bit stack-pointer moves assemble as WZR operations, so generated `mov w0, wsp` / `mov wsp, w0` do not touch WSP
**Root cause:** data_processing.rs:129 compares only the exact name `sp`, so `wsp` falls through to ORR where register 31 is WZR
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:129`
```rust
        if rd_name.to_lowercase() == "sp" || rm_name.to_lowercase() == "sp" {
            let sf = sf_bit(is_64);
            // ADD Xd, Xn, #0: sf 0 0 10001 00 imm12=0 Rn Rd
            let word = ((sf << 31) | (0b10001 << 24)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
```
**Suggested fix:** Treat `wsp` like `sp`.
```rust
        let rd_l = rd_name.to_lowercase();
        let rm_l = rm_name.to_lowercase();
        if rd_l == "sp" || rd_l == "wsp" || rm_l == "sp" || rm_l == "wsp" {
```
**Bug report:** bug_reports/encode_mov_wsp_as_wzr.md
**Repro seed:** rd = 31, rm = 0, is_64 = false, rd_sp = true, rm_sp = false
**Raw output:**
```text
mismatch for mov wsp, w0
left: 704644095 right: 285212703
```

### B2: encode_mov ignores extra operands

**Formal:** ∀ rd,rm ∈ 0..30, extra. encode_mov([Reg("x"+rd), Reg("x"+rm), extra]) = Err(_)
**Contract evidence:** inferred (GNU as rejects extra operands; README.md:12 gas contract)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mov([Reg("x0"), Reg("x0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0xaa0003e0))
**Impact:** Trailing junk after a valid MOV is silently dropped
**Root cause:** data_processing.rs:7 only rejects len < 2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:7`
```rust
    if operands.len() < 2 {
        return Err("mov requires 2 operands".to_string());
    }
```
**Suggested fix:**
```rust
    if operands.len() != 2 {
        return Err("mov requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_mov_extra_operand.md
**Repro seed:** rd = 0, rm = 0, extra = Reg("x0")
**Raw output:**
```text
extra operand must Err, got Ok(Word(2852127712))
```

### B3: encode_mov accepts mixed X/W

**Formal:** ∀ rd,rm ∈ 0..30. encode_mov([Reg("x"+rd), Reg("w"+rm)]) = Err(_)
**Contract evidence:** inferred (GNU as operand mismatch; README.md:12 gas contract)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_mov([Reg("x0"), Reg("w0")])
**Expected / Actual:** Err / Ok(Word(0xaa0003e0))
**Impact:** `mov x0, w0` becomes 64-bit `mov x0, x0`
**Root cause:** data_processing.rs:127 takes sf only from Rd
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:127`
```rust
        let is_64 = is_64bit_reg(rd_name);
```
**Suggested fix:**
```rust
        if is_64bit_reg(rd_name) != is_64bit_reg(rm_name) {
            return Err("mov requires matching register widths".to_string());
        }
```
**Bug report:** bug_reports/encode_mov_mixed_width.md
**Repro seed:** rd = 0, rm = 0
**Raw output:**
```text
mixed width must Err, got Ok(Word(2852127712))
```

### B4: encode_mov encodes FP scalar as integer ORR

**Formal:** ∀ n ∈ 0..31. encode_mov([Reg("d"+n), Reg("d"+(n+1)%32)]) = Err(_)
**Contract evidence:** inferred (GNU as requires SIMD vector element; README.md:12 gas contract)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_mov([Reg("d0"), Reg("d1")])
**Expected / Actual:** Err / Ok(Word(0x2a0103e0))
**Impact:** `mov d0, d1` assembles as `mov w0, w1`
**Root cause:** parse_reg_num accepts `d` prefixes; integer MOV path does not reject FP names
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:123`
```rust
    if let (Some(Operand::Reg(rd_name)), Some(Operand::Reg(rm_name))) = (operands.first(), operands.get(1)) {
        let rd = parse_reg_num(rd_name).ok_or("invalid rd")?;
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
        let is_64 = is_64bit_reg(rd_name);
```
**Suggested fix:**
```rust
        if is_fp_reg(rd_name) || is_fp_reg(rm_name) {
            return Err("integer mov does not accept FP/SIMD registers".to_string());
        }
```
**Bug report:** bug_reports/encode_mov_fp_as_gpr.md
**Repro seed:** fp_n = 0
**Raw output:**
```text
FP scalar mov must Err, got Ok(Word(704709600))
```

### B5: encode_mov encodes mov sp, #imm as MOVZ XZR

**Formal:** ∀ imm. encode_mov([Reg("sp"), Imm(imm)]) = Err(_)
**Contract evidence:** inferred (llvm-mc/gas reject `mov sp, #0`; MOV wide-immediate Rd cannot be SP)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_mov([Reg("sp"), Imm(0)])
**Expected / Actual:** Err / Ok(Word(0xd28003ff))
**Impact:** `mov sp, #0` becomes `mov xzr, #0`
**Root cause:** immediate path maps `sp` to register 31 via get_reg and encodes MOVZ
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:87`
```rust
        let (rd, is_64) = get_reg(operands, 0)?;
        let imm = *imm;
```
**Suggested fix:** Reject SP/WSP as the destination of a wide-immediate MOV.
```rust
        if matches!(operands.first(), Some(Operand::Reg(n)) if {
            let n = n.to_lowercase(); n == "sp" || n == "wsp"
        }) {
            return Err("mov immediate destination cannot be SP".to_string());
        }
```
**Bug report:** bug_reports/encode_mov_sp_imm.md
**Repro seed:** imm = 0
**Raw output:**
```text
mov sp, #imm must Err, got Ok(Word(3531603999))
```

### B6: encode_mov wraps out-of-range NEON lane indices

**Formal:** ∀ vd ∈ 0..31, rm ∈ 0..30, idx ∈ 16..31. encode_mov([RegLane(v_vd, b, idx), Reg("w"+rm)]) = Err(_)
**Contract evidence:** inferred (llvm-mc: vector lane must be in [0, 15] for .b)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_mov([RegLane { v0, b, 16 }, Reg("w0")])
**Expected / Actual:** Err / Ok(Word(0x4e010c00)) INS V0.B[0], W0
**Impact:** Out-of-range lanes silently insert into lane 0
**Root cause:** data_processing.rs:36 uses `*index & 0xF` instead of a range check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:36`
```rust
            "b" => ((*index & 0xF) << 1) | 0b00001,
            "h" => ((*index & 0x7) << 2) | 0b00010,
            "s" => ((*index & 0x3) << 3) | 0b00100,
            "d" => ((*index & 0x1) << 4) | 0b01000,
```
**Suggested fix:** Error when index exceeds the per-size maximum.
```rust
            "b" if *index <= 15 => (*index << 1) | 0b00001,
```
**Bug report:** bug_reports/encode_mov_lane_oob.md
**Repro seed:** vd = 0, rm = 0, idx = 16
**Raw output:**
```text
lane OOB must Err, got Ok(Word(1308695552))
```

### B7: encode_mov accepts mismatched NEON arrangements

**Formal:** ∀ vd,vn ∈ 0..31. encode_mov([RegArrangement(vd,16b), RegArrangement(vn,8b)]) = Err(_)
**Contract evidence:** documented data_processing.rs:11 "mov v1.16b, v0.16b -> ORR v1.16b, v0.16b, v0.16b" (matching T)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mov([RegArrangement(v0,16b), RegArrangement(v0,8b)])
**Expected / Actual:** Err / Ok(Word(0x4ea01c00))
**Impact:** 16b dest with 8b source is encoded as 16-byte ORR
**Root cause:** source arrangement is bound as `_arr_m` and never compared
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:13`
```rust
            Some(Operand::RegArrangement { reg: rm_name, arrangement: _arr_m })) =
        (operands.first(), operands.get(1))
```
**Suggested fix:** Require `arr_d == arr_m` and T ∈ {8b,16b}.
```rust
        if arr_d != arr_m || (arr_d != "8b" && arr_d != "16b") {
            return Err("mov vector arrangements must match 8b or 16b".to_string());
        }
```
**Bug report:** bug_reports/encode_mov_arr_mismatch.md
**Repro seed:** vd = 0, vn = 0
**Raw output:**
```text
16b vs 8b must Err, got Ok(Word(1319115776))
```

### B8: encode_mov encodes 4s vector MOV with Q=0

**Formal:** ∀ vd,vn ∈ 0..31. encode_mov([RegArrangement(vd,4s), RegArrangement(vn,4s)]) = Err(_)
**Contract evidence:** documented README.md:12 gas (gas rejects 4s); data_processing.rs:11 uses 16b for Q=1
**Documentation conflict:** (none) — gas rejects 4s; llvm-mc would use Q=1. SUT Q=0 is wrong under both
**Severity:** high
**Counterexample:** encode_mov([RegArrangement(v0,4s), RegArrangement(v0,4s)])
**Expected / Actual:** Err (gas) / Ok(Word(0x0ea01c00)) Q=0
**Impact:** A 128-bit 4s move only updates the low 64 bits
**Root cause:** Q is 1 only when arr_d == "16b"
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:18`
```rust
        let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:**
```rust
        let q: u32 = match arr_d.as_str() {
            "16b" => 1,
            "8b" => 0,
            _ => return Err(format!("mov vector arrangement must be 8b or 16b, got {arr_d}")),
        };
```
**Bug report:** bug_reports/encode_mov_vec_4s.md
**Repro seed:** vd = 0, vn = 0
**Raw output:**
```text
gas-invalid 4s vector mov must Err, got Ok(Word(245373952))
```

### B9: encode_mov truncates a 64-bit immediate on a W destination

**Formal:** ∀ rd ∈ 0..30, imm with high 32 bits nonzero. encode_mov([Reg("w"+rd), Imm(imm)]) = Err(_)
**Contract evidence:** inferred (GNU as "immediate cannot be moved by a single instruction"; README.md:12 gas contract)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mov([Reg("w0"), Imm(0x0101010101010101)])
**Expected / Actual:** Err / Ok(Word(0x3200c3e0)) ORR W0, WZR, #0x01010101
**Impact:** A 64-bit repeating pattern on W is silently truncated to 32 bits
**Root cause:** encode_bitmask_imm(imm as u64, is_64=false) masks to 32 bits and succeeds
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:110`
```rust
        if let Some((n, immr, imms)) = encode_bitmask_imm(imm as u64, is_64) {
            let sf = sf_bit(is_64);
            // ORR Rd, XZR, #imm: sf 01 100100 N immr imms 11111 Rd
            let word = (sf << 31) | (0b01 << 29) | (0b100100 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (0b11111 << 5) | rd;
```
**Suggested fix:** Reject W dest immediates that are not a 32-bit zero- or sign-extended value.
```rust
        if !is_64 && (imm as u64) > 0xFFFF_FFFF && (imm as i64) != (imm as i32) as i64 {
            return Err("32-bit mov immediate out of range".to_string());
        }
```
**Bug report:** bug_reports/encode_mov_w_large_imm.md
**Repro seed:** rd = 0, imm = 72340172838076673
**Raw output:**
```text
W dest with 64-bit imm must Err, got Ok(Word(838910944))
```

### B10: encode_mov WSP form is not ADD layout (same defect as B1, invariant oracle)

**Formal:** ∀ rd,rm ∈ 0..31, is_64. WSP/SP ⇒ ADD layout (op=0,S=0,opc=10001,imm12=0). else ⇒ ORR layout (opc=01, 01010, N=0, Rn=31)
**Contract evidence:** documented data_processing.rs:129 "Check for MOV to/from SP: uses ADD Xd, Xn, #0"
**Documentation conflict:** data_processing.rs:129 names `sp` only — it does not declare `wsp` invalid
**Severity:** high
**Counterexample:** encode_mov([Reg("wsp"), Reg("w0")]) does not have ADD S=0 (encodes ORR)
**Expected / Actual:** ADD S=0 / S bit reads as 1 (ORR encoding)
**Impact:** Same as B1 — 32-bit SP moves are not ADD
**Root cause:** data_processing.rs:129 compares only the exact name `sp`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:129`
```rust
        if rd_name.to_lowercase() == "sp" || rm_name.to_lowercase() == "sp" {
```
**Suggested fix:** Treat `wsp` like `sp` (same as B1).
```rust
        let rd_l = rd_name.to_lowercase();
        let rm_l = rm_name.to_lowercase();
        if rd_l == "sp" || rd_l == "wsp" || rm_l == "sp" || rm_l == "wsp" {
```
**Bug report:** bug_reports/encode_mov_wsp_add_layout.md
**Repro seed:** rd = 0, imm = 72340172838076673
**Raw output:**
```text
W dest with 64-bit imm must Err, got Ok(Word(838910944))
```

## Design Caveats

- MOVZ vs MOVN vs ORR-bitmask alias encodings of the same immediate follow README.md:287 search order (unshifted MOVZ, unshifted MOVN, then bitmask) rather than llvm-mc preferred shifted MOVZ. User-visible register value agrees. Doc evidence: README.md:287 "first tries single-instruction encodings (MOVZ for 0..0xFFFF, MOVN for bitwise-NOT in 16-bit range, ORR with bitmask immediate...)".
- README.md:287 asserts movz+movk expansion for immediates gas/llvm-mc reject as a single `mov`. That extension is tested by reconstructing the ARM move-wide fields, not by requiring llvm-mc to accept the original `mov`. Doc evidence: README.md:287-291.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_mov_pbt.rs | 15 properties + 7 KAT + 9 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_ -- --test-threads=1
```

Per-bug:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_wsp -- --test-threads=1
cargo test --lib test_encode_mov_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_mov_regression_mixed_width -- --test-threads=1
cargo test --lib test_encode_mov_regression_fp_scalar -- --test-threads=1
cargo test --lib test_encode_mov_regression_sp_imm -- --test-threads=1
cargo test --lib test_encode_mov_regression_lane_oob -- --test-threads=1
cargo test --lib test_encode_mov_regression_arr_mismatch -- --test-threads=1
cargo test --lib test_encode_mov_regression_vec_4s -- --test-threads=1
cargo test --lib test_encode_mov_regression_w_large_imm -- --test-threads=1
```

Build command as run: `cargo test --lib encode_mov_ -- --test-threads=1` in `/home/toan/github/claudes-c-compiler`.

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_mov_wsp_as_wzr.md (+ .html)
- pbt-out/bug_reports/encode_mov_extra_operand.md (+ .html)
- pbt-out/bug_reports/encode_mov_mixed_width.md (+ .html)
- pbt-out/bug_reports/encode_mov_fp_as_gpr.md (+ .html)
- pbt-out/bug_reports/encode_mov_sp_imm.md (+ .html)
- pbt-out/bug_reports/encode_mov_lane_oob.md (+ .html)
- pbt-out/bug_reports/encode_mov_arr_mismatch.md (+ .html)
- pbt-out/bug_reports/encode_mov_vec_4s.md (+ .html)
- pbt-out/bug_reports/encode_mov_w_large_imm.md (+ .html)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 11:13 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 179/307 total | PBT candidates: 179 | Tested: 179 (100%) | 1 pass, 179 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 179 |
| **Tested (of PBT candidates)** | **179 / 179 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 179 / -1 |
| **Overall (tested / all functions)** | **179 / 307 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 179 | 179 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 179 | 179 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 18 | 18 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_ubfx | bitfield.rs |
| encode_ubfm | bitfield.rs |
| encode_sbfx | bitfield.rs |
| encode_sbfm | bitfield.rs |
| encode_sbfiz | bitfield.rs |
| encode_shift | gp_integer.rs |
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
| encode_adc | data_processing.rs |
| encode_adr | load_store.rs |
| encode_bic | data_processing.rs |
| encode_neon_three_diff_narrow | neon.rs |
| encode_bics | data_processing.rs |
| encode_bl | compare_branch.rs |
| encode_blr | compare_branch.rs |
| encode_br | compare_branch.rs |
| encode_branch | compare_branch.rs |
| encode_cbz | compare_branch.rs |
| encode_ccmp_ccmn | compare_branch.rs |
| encode_cinc | compare_branch.rs |
| encode_cinv | compare_branch.rs |
| encode_cmn | compare_branch.rs |
| encode_cmp | compare_branch.rs |
| encode_cneg | compare_branch.rs |
| encode_csel | compare_branch.rs |
| encode_cset | compare_branch.rs |
| encode_csetm | compare_branch.rs |
| encode_csinc | compare_branch.rs |
| encode_csinv | compare_branch.rs |
| encode_csneg | compare_branch.rs |
| encode_div | data_processing.rs |
| encode_eon | data_processing.rs |
| encode_ldar_stlr | load_store.rs |
| encode_neon_across_long | neon.rs |
| encode_neon_float_cmp_zero | neon.rs |
| encode_neon_sli | neon.rs |
| encode_ldur_stur | load_store.rs |
| encode_ldxp_stxp | load_store.rs |
| encode_neon_float_three_same | neon.rs |
| encode_ldxr_stxr | load_store.rs |
| encode_logical | data_processing.rs |
| encode_madd | data_processing.rs |
| encode_movk | data_processing.rs |
| encode_movn | data_processing.rs |
| encode_movz | data_processing.rs |
| encode_neon_qshrn | neon.rs |
| encode_msub | data_processing.rs |
| encode_mul | data_processing.rs |
| encode_mvn | data_processing.rs |
| encode_neon_shift_right | neon.rs |
| encode_neg | pseudo.rs |
| encode_negs | data_processing.rs |
| encode_neon_shift_imm | neon.rs |
| encode_neon_tbl | neon.rs |
| encode_orn | data_processing.rs |
| encode_ret | compare_branch.rs |
| encode_sbc | data_processing.rs |
| encode_neon_shll | neon.rs |
| encode_neon_sqshrun | neon.rs |
| encode_smull | data_processing.rs |
| encode_sxth | data_processing.rs |
| encode_sxtw | data_processing.rs |
| encode_neon_shift_left_imm | neon.rs |
| encode_umaddl | data_processing.rs |
| encode_umulh | data_processing.rs |
| encode_neon_rbit | neon.rs |
| encode_umull | data_processing.rs |
| encode_uxtw | data_processing.rs |
| encode_ldaxr_stlxr | load_store.rs |
| encode_ldrsw | load_store.rs |
| encode_ldtr_sized | load_store.rs |
| encode_prfm | load_store.rs |
| encode_smulh | data_processing.rs |
| encode_fcvt_rounding | fp_scalar.rs |
| encode_fp_1src | fp_scalar.rs |
| encode_int_to_float | fp_scalar.rs |
| encode_fcmp | fp_scalar.rs |
| encode_fcvt_precision | fp_scalar.rs |
| encode_neon_aes | neon.rs |
| encode_bfi | bitfield.rs |
| encode_bfxil | bitfield.rs |
| encode_cas | load_store.rs |
| encode_cls | bitfield.rs |
| encode_clz | bitfield.rs |
| encode_extr | bitfield.rs |
| encode_fmov | fp_scalar.rs |
| encode_fp_arith | fp_scalar.rs |
| encode_rbit | bitfield.rs |
| encode_rev | bitfield.rs |
| encode_rev16 | bitfield.rs |
| encode_rev32 | bitfield.rs |
| encode_ubfiz | bitfield.rs |
| encode_bfm | bitfield.rs |
| encode_neon_float_two_misc | neon.rs |
| encode_fabs | fp_scalar.rs |
| encode_fmadd_fmsub | fp_scalar.rs |
| encode_fnmadd_fnmsub | fp_scalar.rs |
| encode_fneg | fp_scalar.rs |
| encode_fsqrt | fp_scalar.rs |
| encode_neon_dup | neon.rs |
| encode_ldrs | load_store.rs |
| encode_neon_ldnr | neon.rs |
| encode_neon_ld1r | neon.rs |
| encode_neon_ld_st_single | neon.rs |
| encode_neon_ld_st_multi | neon.rs |
| encode_neon_tbx | neon.rs |
| encode_neon_ins | neon.rs |
| encode_neon_umov | neon.rs |
| encode_neon_ext | neon.rs |
| encode_neon_movi | neon.rs |
| encode_neon_mvni | neon.rs |
| encode_cnt | neon.rs |
| encode_neon_not | neon.rs |
| encode_neon_rev64 | neon.rs |
| encode_neon_bsl | neon.rs |
| encode_neon_addv | neon.rs |
| encode_neon_across | neon.rs |
| encode_neon_zip_uzp | neon.rs |
| encode_neon_eor3 | neon.rs |
| encode_neon_pmull | neon.rs |
| encode_neon_add_sub | neon.rs |
| encode_neon_ushr | neon.rs |
| encode_neon_sshr | neon.rs |
| encode_neon_shl | neon.rs |
| encode_neon_sri | neon.rs |
| encode_neon_shrn | neon.rs |
| encode_neon_two_misc | neon.rs |
| encode_neon_xtl | neon.rs |
| encode_neon_mul | neon.rs |
| encode_neon_pmul | neon.rs |
| encode_neon_mla | neon.rs |
| encode_neon_mls | neon.rs |
| encode_neon_three_same | neon.rs |
| encode_neon_three_diff | neon.rs |
| encode_neon_logical | neon.rs |
| encode_neon_cmp_zero | neon.rs |
| encode_neon_elem_long | neon.rs |
| encode_neon_elem | neon.rs |
| encode_neon_float_elem | neon.rs |
| encode_neon_fcvtl | neon.rs |
| encode_neon_fcvtn | neon.rs |
| encode_neon_bitwise_insert | neon.rs |
| encode_neon_faddp | neon.rs |
| encode_neon_scalar_three_same | neon.rs |
| encode_neon_scalar_addp | neon.rs |
| encode_neon_scalar_two_misc | neon.rs |
| encode_neon_scalar_qshrn | neon.rs |
| encode_neon_two_misc_narrow | neon.rs |
| encode_dmb | system.rs |
| encode_dsb | system.rs |
| encode_mrs | system.rs |
| encode_msr | system.rs |
| encode_svc | system.rs |
| encode_hvc | system.rs |
| encode_brk | system.rs |
| encode_hint | system.rs |
| encode_bti | system.rs |
| encode_ic | system.rs |
| encode_dc | system.rs |
| encode_sys | system.rs |
| encode_at | system.rs |
| encode_tlbi | system.rs |
| encode_swp | load_store.rs |
| encode_ldop | load_store.rs |
| encode_stop | load_store.rs |
| encode_tst | compare_branch.rs |
| encode_tbz | compare_branch.rs |
| encode_crc32 | bitfield.rs |
| encode_smaddl | data_processing.rs |
| encode_mneg | data_processing.rs |
| encode_sxtb | data_processing.rs |
| encode_uxth | data_processing.rs |
| encode_uxtb | data_processing.rs |
| encode_ldr_str | load_store.rs |
| encode_ldp_stp | load_store.rs |
| encode_ldnp_stnp | load_store.rs |
| encode_adrp | load_store.rs |
| encode_mov | data_processing.rs |
