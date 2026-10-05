# PBT Campaign Report: encode_neon_tbx

## Summary

**Verdict:** 10 medium: encode_neon_tbx silently encodes illegal TBX forms (extra operand, Ta≠8b/16b, nregs>4, non-sequential tables, table≠.16b, GPR dest/Vm, bare list member, mismatched Ta) and panics on an empty register list, so GNU-style `tbx` that llvm-mc/gas reject becomes a wrong 32-bit word or a crash.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_tbx
**Tests:** 10 properties (plus 1 KAT + 10 regression witnesses)
**Result:** 5 passing, 5 failing properties, 10 bugs
**Change surface:** 1 changed function (encode_neon_tbx), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (Rust cargo test, C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (list/Vm kinds + uppercase spellings).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_tbx | 10 properties (5 pass / 5 fail) + 1 KAT + 10 regressions | 10 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_tbx ignores a surplus fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, ta ∈ {8b,16b}, n ∈ {1,2,3,4}. llvm-mc rejects 4-operand tbx ⇒ encode_neon_tbx(ops++[Vextra.ta]) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; llvm-mc rejects `tbx v0.8b, {v0.16b}, v0.8b, v0.8b`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, {v0.16b}, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** A trailing extra token is assembled as the 3-operand form instead of an assembler error.
**Root cause:** neon.rs:804 `if operands.len() < 3` only rejects too few operands; extras past index 2 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:804`
```rust
    if operands.len() < 3 {
        return Err("tbx requires 3 operands".to_string());
    }
```
**Suggested fix:** Require exactly three operands.
```rust
    if operands.len() != 3 {
        return Err("tbx requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_tbx_extra_operand.md
**Repro seed:** rd=0, rn=0, rm=0, extra=0, ta="8b", n=1
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:574:5:
tbx v0.8b, {v0.16b}, v0.8b, v0.8b must Err (exactly 3 operands)
```

### B2: encode_neon_tbx encodes Ta other than 8b/16b as Q=0

**Formal:** ∀ rd,rn,rm ∈ {0..31}, ta ∈ {4h,8h,2s,4s,2d,1d}, n ∈ {1,2,3,4}. llvm-mc rejects tbx Vd.ta ⇒ encode_neon_tbx is Err
**Contract evidence:** inferred (README.md:12; ARM TBX Ta ∈ {8B,16B}; llvm-mc rejects `tbx v0.4h, {v0.16b}, v0.4h`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.4h, {v0.16b}, v0.4h])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** An illegal arrangement is silently rewritten as Q=0 (8b) TBX.
**Root cause:** neon.rs:808 sets Q=1 only for the string "16b" and otherwise Q=0, with no arrangement whitelist.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:808`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Reject any Ta other than 8b/16b.
```rust
    let q: u32 = match arr_d.as_str() {
        "16b" => 1,
        "8b" => 0,
        other => return Err(format!("tbx: Ta must be 8b or 16b, got {}", other)),
    };
```
**Bug report:** bug_reports/encode_neon_tbx_invalid_ta.md
**Repro seed:** rd=0, rn=0, rm=0, ta="4h", n=1
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_invalid_ta' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:584:5:
tbx v0.4h, {v0.16b}, v0.4h must Err (Ta not 8b/16b)
```

### B3: encode_neon_tbx panics on an empty register list

**Formal:** ∀ invalid table (empty | n∈{5..8} | non-sequential pair | arrangement ∉ {16b}). encode_neon_tbx is Err (not panic, not Ok)
**Contract evidence:** inferred (signature returns Result; indexing regs[0] on an empty list panics instead of Err)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, RegList([]), v0.8b])
**Expected / Actual:** Err / panic `index out of bounds: the len is 0 but the index is 0`
**Impact:** An empty `{ }` table operand crashes the assembler instead of reporting an error.
**Root cause:** neon.rs:812 indexes `regs[0]` without checking `regs.is_empty()`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:812`
```rust
            let first_reg = match &regs[0] {
```
**Suggested fix:** Reject an empty list before indexing.
```rust
            if regs.is_empty() {
                return Err("tbx: expected register in list".to_string());
            }
            let first_reg = match &regs[0] {
```
**Bug report:** bug_reports/encode_neon_tbx_empty_list_panic.md
**Repro seed:** kind=0, rd=0, rn=0, rm=0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_empty_list' panicked at src/backend/arm/assembler/encoder/neon.rs:812:40:
index out of bounds: the len is 0 but the index is 0
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_empty_list' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:603:19:
empty table list must Err, not panic on regs[0]
```

### B4: encode_neon_tbx wraps table lists longer than 4 into len&3

**Formal:** ARM TBX allows 1–4 table registers; llvm-mc rejects more, so encode_neon_tbx with nregs>4 = Err
**Contract evidence:** inferred (README.md:12; llvm-mc "invalid number of vectors" for 5-register TBX)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, {v0.16b..v4.16b}, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e001000)) encoded as 1-register TBX
**Impact:** Five table vectors silently become a 1-register lookup (len=(5-1)&3=0).
**Root cause:** neon.rs:823 masks `(num_regs - 1) & 0x3` instead of rejecting nregs outside 1..=4.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:823`
```rust
    let len = (num_regs - 1) & 0x3;
```
**Suggested fix:** Reject nregs outside 1..=4 before encoding len.
```rust
    if !(1..=4).contains(&num_regs) {
        return Err(format!("tbx: invalid number of vectors: {}", num_regs));
    }
    let len = num_regs - 1;
```
**Bug report:** bug_reports/encode_neon_tbx_five_regs.md
**Repro seed:** n=5, rd=0, rn=0, rm=0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_five_regs' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:611:5:
tbx v0.8b, {v0.16b..v4.16b}, v0.8b must Err (invalid number of vectors)
```

### B5: encode_neon_tbx ignores non-sequential table register names

**Formal:** ARM TBX table registers must be consecutive wrapping v0–v31; llvm-mc rejects gaps, so encode_neon_tbx({v0.16b, v2.16b}) = Err
**Contract evidence:** inferred (README.md:12; llvm-mc "registers must be sequential")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, {v0.16b, v2.16b}, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e003000))
**Impact:** A gap in the list is accepted and the encoding names the wrong second register (v1, not v2).
**Root cause:** neon.rs:812-817 only reads regs[0] and regs.len(); later names and sequentiality are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:812`
```rust
            let first_reg = match &regs[0] {
                Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid reg")?,
                Operand::Reg(name) => parse_reg_num(name).ok_or("invalid reg")?,
                _ => return Err("tbx: expected register in list".to_string()),
            };
            (first_reg, regs.len() as u32)
```
**Suggested fix:** Require each subsequent list member to be (first+i) mod 32.
```rust
            for (i, r) in regs.iter().enumerate() {
                let num = /* parse Vn.16b */;
                if num != (first_reg + i as u32) % 32 {
                    return Err("tbx: registers must be sequential".to_string());
                }
            }
```
**Bug report:** bug_reports/encode_neon_tbx_nonsequential.md
**Repro seed:** {v0.16b, v2.16b}
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_nonsequential' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:625:5:
tbx v0.8b, {v0.16b, v2.16b}, v0.8b must Err (registers must be sequential)
```

### B6: encode_neon_tbx accepts a table arrangement other than .16b

**Formal:** ARM TBX table registers are .16B; llvm-mc rejects `{v0.8b}`, so encode_neon_tbx with table T≠16b = Err
**Contract evidence:** inferred (README.md:12; llvm-mc "invalid operand for instruction" on `{v0.8b}`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, {v0.8b}, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** A `.8b` table is silently rewritten as `.16b`.
**Root cause:** neon.rs:813 binds `reg, ..` and never inspects the table arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:813`
```rust
                Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid reg")?,
```
**Suggested fix:** Require arrangement == "16b" on every list member.
```rust
                Operand::RegArrangement { reg, arrangement } if arrangement == "16b" => {
                    parse_reg_num(reg).ok_or("invalid reg")?
                }
```
**Bug report:** bug_reports/encode_neon_tbx_table_not_16b.md
**Repro seed:** table arr="8b"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_table_not_16b' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:639:5:
tbx v0.8b, {v0.8b}, v0.8b must Err (table must be .16b)
```

### B7: encode_neon_tbx encodes a GPR/FP dest as Vd

**Formal:** TBX dest must be Vd.Ta; llvm-mc rejects `tbx x0, {v0.16b}, v0.8b`, so encode_neon_tbx([Reg("x0"), ...]) = Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects GPR dest)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([Reg("x0"), {v0.16b}, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** A GPR dest is assembled as `tbx v0.8b, ...` because parse_reg_num maps x0→0.
**Root cause:** neon.rs:807 calls get_neon_reg, which accepts Operand::Reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:807`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Operand::RegArrangement with a v-prefix dest and Ta in {8b,16b}.
```rust
    let (rd, arr_d) = match &operands[0] {
        Operand::RegArrangement { reg, arrangement } => {
            (parse_reg_num(reg).ok_or("invalid NEON register")?, arrangement.clone())
        }
        other => return Err(format!("tbx: dest must be Vd.Ta, got {:?}", other)),
    };
```
**Bug report:** bug_reports/encode_neon_tbx_gpr_dest.md
**Repro seed:** dest="x0"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_gpr_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:653:5:
tbx x0, {v0.16b}, v0.8b must Err (GPR dest is not Vd.Ta)
```

### B8: encode_neon_tbx ignores Vm.Ta vs Vd.Ta mismatch

**Formal:** ARM TBX requires Vm.Ta = Vd.Ta; llvm-mc rejects `tbx v0.8b, {v0.16b}, v0.16b`, so encode_neon_tbx with mismatched Ta = Err
**Contract evidence:** inferred (README.md:12; llvm-mc "invalid operand for instruction")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, {v0.16b}, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** A mismatched index arrangement is silently accepted; only Vd's Q bit is encoded.
**Root cause:** neon.rs:822 binds `(rm, _)` and discards Vm's arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:822`
```rust
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Compare Vm arrangement to Vd's Ta.
```rust
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_m != arr_d {
        return Err(format!("tbx: Vm.Ta ({}) must match Vd.Ta ({})", arr_m, arr_d));
    }
```
**Bug report:** bug_reports/encode_neon_tbx_mismatched_t.md
**Repro seed:** Vd.8b vs Vm.16b
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_mismatched_t' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:695:5:
tbx v0.8b, {v0.16b}, v0.16b must Err (Vd.Ta must equal Vm.Ta)
```

### B9: encode_neon_tbx accepts a bare Reg as a table-list member

**Formal:** Each TBX table member must be Vn.16B; llvm-mc rejects `{v0}`, so encode_neon_tbx with Operand::Reg in the list = Err
**Contract evidence:** inferred (README.md:12; ARM table members are Vn.16B)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, RegList([Reg("v0")]), v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** A missing arrangement specifier is silently filled in as .16b.
**Root cause:** neon.rs:814 accepts Operand::Reg in the table list via parse_reg_num.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:814`
```rust
                Operand::Reg(name) => parse_reg_num(name).ok_or("invalid reg")?,
```
**Suggested fix:** Accept only RegArrangement with .16b in the list.
```rust
                Operand::Reg(_) => return Err("tbx: table member must be Vn.16b".to_string()),
```
**Bug report:** bug_reports/encode_neon_tbx_bare_list_reg.md
**Repro seed:** list member Reg("v0")
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_bare_list_reg' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:667:5:
tbx v0.8b, {v0}, v0.8b must Err (table member is not Vn.16B)
```

### B10: encode_neon_tbx encodes a bare GPR as Vm

**Formal:** TBX Vm must be Vm.Ta; llvm-mc rejects `tbx v0.8b, {v0.16b}, x0`, so encode_neon_tbx with Operand::Reg Vm = Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects GPR Vm)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_tbx([v0.8b, {v0.16b}, Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x0e001000))
**Impact:** `tbx v0.8b, {v0.16b}, x0` is assembled as `tbx v0.8b, {v0.16b}, v0.8b`.
**Root cause:** neon.rs:822 calls get_neon_reg, which accepts Operand::Reg for Vm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:822`
```rust
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Operand::RegArrangement for Vm with Ta matching Vd.
```rust
    let (rm, arr_m) = match &operands[2] {
        Operand::RegArrangement { reg, arrangement } => {
            (parse_reg_num(reg).ok_or("invalid NEON register")?, arrangement.clone())
        }
        other => return Err(format!("tbx: Vm must be Vm.Ta, got {:?}", other)),
    };
```
**Bug report:** bug_reports/encode_neon_tbx_bare_vm.md
**Repro seed:** kind=3, prefix=x, n=0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_bare_vm' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:681:5:
tbx v0.8b, {v0.16b}, x0 must Err (GPR Vm is not Vm.Ta)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs | 10 properties + 1 KAT + 10 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_tbx_pbt` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_tbx -- --test-threads=1
```

Per-bug regression (each fails while the bug is unfixed):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_invalid_ta -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_empty_list -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_five_regs -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_nonsequential -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_table_not_16b -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_gpr_dest -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_mismatched_t -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_bare_list_reg -- --test-threads=1
cargo test --lib test_encode_neon_tbx_regression_bare_vm -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign phases
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/report.json — machine-readable report
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/FUNCTION_INDEX.md — function index (encode_neon_tbx marked yes)
- pbt-out/bug_reports/encode_neon_tbx_extra_operand.md + .html
- pbt-out/bug_reports/encode_neon_tbx_invalid_ta.md + .html
- pbt-out/bug_reports/encode_neon_tbx_empty_list_panic.md + .html
- pbt-out/bug_reports/encode_neon_tbx_five_regs.md + .html
- pbt-out/bug_reports/encode_neon_tbx_nonsequential.md + .html
- pbt-out/bug_reports/encode_neon_tbx_table_not_16b.md + .html
- pbt-out/bug_reports/encode_neon_tbx_gpr_dest.md + .html
- pbt-out/bug_reports/encode_neon_tbx_mismatched_t.md + .html
- pbt-out/bug_reports/encode_neon_tbx_bare_list_reg.md + .html
- pbt-out/bug_reports/encode_neon_tbx_bare_vm.md + .html
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_tbx_pbt.txt — proptest failure seeds

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 09:27 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 106/289 total | PBT candidates: 106 | Tested: 106 (100%) | 0 pass, 106 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 106 |
| **Tested (of PBT candidates)** | **106 / 106 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 106 / 0 |
| **Overall (tested / all functions)** | **106 / 289 (37%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 106 | 106 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 106 | 106 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
| neon.rs | 68 | 21 | 21 | 100% | covered |
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
| encode_fneg | fp_scalar.rs |
| encode_fsqrt | fp_scalar.rs |
| encode_neon_dup | neon.rs |
| encode_ldrs | load_store.rs |
| encode_neon_ldnr | neon.rs |
| encode_neon_ld1r | neon.rs |
| encode_neon_ld_st_single | neon.rs |
| encode_neon_ld_st_multi | neon.rs |
| encode_neon_tbx | neon.rs |
