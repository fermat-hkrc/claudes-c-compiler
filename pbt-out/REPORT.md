# PBT Campaign Report: encode_inc_dec (i686)

## Summary

**Verdict:** 3 bugs (1 high, 2 medium): `encode_inc_dec` drops segment overrides on memory forms (`incl %es:(%eax)` → bare `incl (%eax)`), and silently accepts non-GP (`%xmm0`) and width-mismatched (`incl %ax`) registers as GP encodings.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_inc_dec (src/backend/i686/assembler/encoder/gp_integer.rs)
**Tests:** 10 properties (+ KAT + 3 regression witnesses)
**Result:** 6 passing, 4 failing properties → 3 distinct SUT bugs
**Change surface:** 1 changed function (encode_inc_dec), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — Rust cargo tests; `coverage_gaps` used after first full run for contract-surface sweep (standard tier, 1 round)
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_inc_dec | 10 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: Missing segment-override prefix on memory INC/DEC

**Formal:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base ∈ GP32, op ∈ {incl,decl}. encode(op, seg:(base)) = llvm_mc(op %seg:(%base))
**Contract evidence:** documented core.rs:31-42 `emit_segment_prefix` for es/cs/ss/ds/fs/gs; differential vs llvm-mc i686
**Documentation conflict:** (none — helper exists and is used by sibling encoders; encode_inc_dec simply never calls it)
**Severity:** high
**Counterexample:** `incl %es:(%eax)` then compare bytes — expected `[0x26,0xff,0x00]`, actual `[0xff,0x00]`
**Expected / Actual:** `[0x26, 0xff, 0x00]` / `[0xff, 0x00]`
**Impact:** Segment overrides silently dropped; TLS/far-data/kernel asm that uses `%fs:`/`%gs:`/`%es:` on INC/DEC targets the wrong segment.
**Root cause:** gp_integer.rs:829-832 memory arm never calls `self.emit_segment_prefix(mem)` before FE/FF.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:829`
```rust
            Operand::Memory(mem) => {
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xFE } else { 0xFF });
                self.encode_modrm_mem(op_ext, mem)
            }
```
**Suggested fix:** Emit the segment override first, then optional 0x66, then opcode.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xFE } else { 0xFF });
                self.encode_modrm_mem(op_ext, mem)
            }
```
**Bug report:** bug_reports/encode_inc_dec_missing_segment_prefix.md
**Repro seed:** proptest cc 31094b365406352a9df2404794df9dcacdd95b2152686b8205e1e77f61ee1b5a (seg=es, base=eax, disp=0, mnem=incl)
**Raw output:**
```text
segment diff `incl %es:(%eax)`: sut=[ff, 00] mc=[26, ff, 00]
```

### B2: Non-GP registers accepted via reg_num alias

**Formal:** ∀ x ∈ XMM, op ∈ {incl,decl}. encode(op, x) = Err
**Contract evidence:** inferred (Intel SDM INC/DEC r/m GP only; llvm-mc rejects `incl %xmm0`; registers.rs:4-15 aliases xmm→num)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `incl %xmm0` → `Ok([0x40])` (same as `incl %eax`)
**Expected / Actual:** Err / Ok([0x40])
**Impact:** Invalid operand silently encodes as a different GP register.
**Root cause:** gp_integer.rs:811-812 uses bare `reg_num` with no GP-only check; xmm0 maps to 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:811`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                if size == 4 {
                    // Use compact single-byte encoding: 0x40+reg (inc) or 0x48+reg (dec)
                    let base = if op_ext == 0 { 0x40 } else { 0x48 };
                    self.bytes.push(base + num);
```
**Suggested fix:** Reject xmm/mm/st/ymm (and other non-GP) before encoding.
```rust
                if is_xmm(&reg.name) || is_mm(&reg.name) || reg.name.starts_with("st")
                    || reg.name.starts_with("ymm")
                {
                    return Err(format!("inc/dec does not accept register %{}", reg.name));
                }
```
**Bug report:** bug_reports/encode_inc_dec_accepts_non_gp.md
**Repro seed:** x=xmm0, mnem=incl
**Raw output:**
```text
encode_inc_dec must reject non-GP `xmm0` for incl, got Ok([64])
```

### B3: Width-mismatched GP registers accepted

**Formal:** ∀ mismatched (mnem, reg) pairs (e.g. incl+%ax). encode = Err
**Contract evidence:** inferred (AT&T size suffix must match register width; llvm-mc rejects `incl %ax`; `reg_size` exists at registers.rs:63-70)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `incl %ax` → `Ok([0x40])` (same as `incl %eax`)
**Expected / Actual:** Err / Ok([0x40])
**Impact:** Assembler accepts invalid width combinations and emits a different-sized operation than written.
**Root cause:** gp_integer.rs:811 never checks `reg_size(&reg.name) == size`; ax and eax share reg_num 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:811`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                if size == 4 {
                    // Use compact single-byte encoding: 0x40+reg (inc) or 0x48+reg (dec)
                    let base = if op_ext == 0 { 0x40 } else { 0x48 };
                    self.bytes.push(base + num);
```
**Suggested fix:** Require `reg_size(&reg.name) == size`.
```rust
                if reg_size(&reg.name) != size {
                    return Err(format!(
                        "register %{} width does not match inc/dec size {size}",
                        reg.name
                    ));
                }
```
**Bug report:** bug_reports/encode_inc_dec_mismatched_width.md
**Repro seed:** case=0, r16=ax, mnem=incl
**Raw output:**
```text
encode_inc_dec must reject width-mismatched `incl %ax`, got Ok([64])
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs | 10 properties + 7 KAT + 3 regressions |
| src/backend/i686/assembler/encoder/mod.rs | +`#[cfg(test)] mod encode_inc_dec_pbt;` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec -- --test-threads=1
```

B1 regression:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec_regression_missing_es_prefix -- --test-threads=1
```

B2 regression:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec_regression_xmm0_accepted -- --test-threads=1
```

B3 regression:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec_regression_mismatched_width -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_inc_dec_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_inc_dec_accepts_non_gp.md (+ .html)
- pbt-out/bug_reports/encode_inc_dec_mismatched_width.md (+ .html)
- pbt-out/run/encode_inc_dec_test.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 07:56 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 282/399 total | PBT candidates: 282 | Tested: 282 (100%) | 1 pass, 282 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 399 |
| PBT candidates (from FUNCTION_INDEX) | 282 |
| **Tested (of PBT candidates)** | **282 / 282 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 282 / -1 |
| **Overall (tested / all functions)** | **282 / 399 (71%)** |
| Untested | 0 |
| Skipped | 0 |

## Mo