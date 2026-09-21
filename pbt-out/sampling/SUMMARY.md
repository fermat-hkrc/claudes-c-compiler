# ccc Issue-Tracker Verification Report — Full Run (all 509 issues)

**Repository:** `fermat-hkrc/claudes-c-compiler` (branch `explore/pbt-test`, commit `bc6f5170`)
**Tracker scope:** all 509 open issues (closed #1 meta-stub and #511 duplicate excluded)
**Verification method:** Contract-Based Differential Validation (LLM agent) — every issue tested live, no sampling or extrapolation
**References:** gcc 13.3 (`aarch64-linux-gnu-gcc` / gas 2.42, warnings included), clang / llvm-mc (secondary), `qemu-i386` (i686 runtime), host gcc + qemu (x86-64 C level)
**Verification date:** 2026-09-15 · Artifacts: `pbt-out/verified_bug/` (38 reports), `pbt-out/full_verification_tracker.md` (509-row cross-check), `pbt-out/manual_tests.md`

---

## 1. Executive Summary

| Headline | Result |
|---|---|
| Coverage | **509/509 issues verified live** (503 ARM issues via the ccc-arm / gcc / clang command-line pipeline with `objdump` encoding comparison; 6 C-level and encoding issues verified individually) |
| **Defect realness** | **506 real (99.4%) · 3 false positives (0.6%, user-confirmed)** |
| Issue-text fidelity | 3 wording/severity amendments (#17, #150, #497); defects stand |
| Dominant compiler failure | Severity class S2 "invalid input silently accepted and encoded": 459/509 (90.2%) — invalid input is turned into machine code with zero diagnostics |
| Most harmful class | Severity class S1 "silent wrong code on valid input" (20 issues) — including the family of 9 issues where valid half-precision (FP16) floating-point instructions are silently encoded as single-precision instructions |
| Crash exposure | **15 crash-capable paths**: 9 panics reachable through the command-line interface + 6 latent encoder panics that an earlier parser stage or warning path hides |
| Fix leverage | Two shared helper functions (`get_gpr_checked`, `check_arity`) mechanically prevent ~60 issues; one line fixes the 9-issue half-precision encoding family |

**Bottom line:** the FM-Agent (property-based testing campaign) issue tracker is highly trustworthy — 506 of 509
filed reports correspond to reproducible defects cross-confirmed against the GNU and LLVM toolchains
(the 3 exceptions are alias-syntax enhancement requests valid only for llvm-mc, user-confirmed). The defining weakness of ccc
is not crashing but **diagnostic silence**: 479 of 506 real defects (94.7%) emit machine code with zero diagnostics —
459 (S2) encode invalid input that a reference toolchain rejects; 20 (S1) encode valid input wrongly where the references
encode it correctly.

---

## 2. Verification Methodology

Each issue's stated **Law** (contract) was compiled into a Hoare-triple probe
`{malformed input} encode {Err / expected-word}` and evaluated dynamically:

1. **Unit witness** — direct call to the encoder function under test (scratch test, kept as regression test)
2. **Reference arbitration** — the same assembly input assembled by gcc/gas **and** clang/llvm-mc;
   acceptance, error text, warnings, and emitted encodings all recorded
3. **Differential decode** — for accepted inputs, encodings extracted via `objdump` and compared word-for-word
4. **CONSTRAINED UNPREDICTABLE adjudication** — for architecturally CONSTRAINED UNPREDICTABLE constructs, both references'
   stances checked (gas accepts with a warning; llvm-mc rejects) before classifying

Every verdict is backed by a reproducible witness; nothing is judged by inspection alone.

---

## 3. Full-Tracker Statistics (n = 509)

### 3.1 Classification by ccc subsystem (where in the compiler the defect lives)

Taxonomy is **compiler-internal**: each real defect is attributed to the ccc layer whose missing
check or wrong bit-packing produces it (paths under `src/`). CWE labels are deliberately dropped —
they collapse ~76% of the tracker into "CWE-20 Improper Input Validation" and say nothing about
what the compiler *does* (see 3.2 for the severity classification).

| Layer | Missing / wrong mechanism in ccc | ccc code location | Real defects | % of 509 |
|---|---|---|---|---|
| L1 Operand-count validation | extra operands, or wrong register-list cardinality, consumed silently | per-mnemonic handlers, `src/backend/arm/assembler/encoder/*.rs` | 91 | 17.9% |
| L2 Register class and width validation | floating-point/SIMD/stack-pointer registers accepted where a general-purpose register is required; 32-bit and 64-bit widths mixed | `encoder/data_processing.rs`, `encoder/load_store.rs` | 207 | 40.7% |
| L3 NEON arrangement consistency | destination arrangement / register-shape mismatch left unvalidated | `encoder/neon.rs` | 35 | 6.9% |
| L4 Stack-pointer / zero-register identity in encoding slot 31 | `sp`/`wsp`/`xzr`/`wzr` accepted where the other is required (bit 31 mis-encoded) | register-field packing across encoders | 82 | 16.1% |
| L5 Immediate range validation vs bit-masking | out-of-range immediates masked (`& 0x1F`, `as u32`) instead of rejected; truncation wraps | immediate packing across encoders | 52 | 10.2% |
| L6 Shift/extend operator validation | wrong operator kind (`ror`, `lsr`, `lslx`) or amount beyond the register width accepted | shift-operand decoding | 22 | 4.3% |
| L7 Relocation-modifier grammar | `:lo12:` accepted where a label is required | `src/backend/arm/assembler/parser.rs` | 3 | 0.6% |
| L8 Branch and label grammar (rejects valid input) | immediate branch offsets refused (symbol operands only) | `encoder/compare_branch.rs`, parser | 5 | 1.0% |
| L9 CONSTRAINED UNPREDICTABLE diagnostics | gas-style warnings absent for CONSTRAINED UNPREDICTABLE forms | diagnostics layer | 3 | 0.6% |
| L10 Non-assembler subsystems | C frontend 4 (#2 #3 #4 #510) · source decoding 1 (#508) · linker/driver 1 (#509) | `src/backend/cast.rs` + `f128_softfloat.rs`, frontend lexer, `src/driver` | 6 | 1.2% |
| **Subtotal (real defects)** | | | **506** | **99.4%** |
| False positive | **no defect** — alias syntax accepted only by llvm-mc (#30 #119 #247); ccc matches gas 2.42, which also rejects | — | **3** | **0.6%** |
| **Total** | | | **509** | **100%**† |

† percentages rounded to 0.1; the column sums to 100.1% due to rounding.

One classifier, each issue counted once. The 6 missing-warning issues (see 3.3) sit under their
operand-level layer (L2/L5) except 3 whose missing mechanism is purely the diagnostic (L9).

### 3.2 ccc severity ladder — by what the compiler ships (replaces the CWE roll-up)

Severity is defined by **compiler outcome**, not vulnerability taxonomy: what does ccc hand the
user for this input? Ranked worst-first:

| Severity | Compiler outcome (ccc behavior) | Definition | Issues | % of 509 |
|---|---|---|---|---|
| **S1** | **Silent wrong code on valid input** | valid source → wrong machine word, no diagnostic (17 ARM differential-encoding issues #15 #139 #264 #344 #354 #355 #358 #361 #366 #409 #414 #475 #478 #481 #484 #504 #505, plus C-level #2 #4 #508) | 20 | 3.9% |
| **S2** | **Invalid input silently accepted and encoded** | invalid input silently turned into machine code — composed of 453 silent acceptances (3.3, including #507 whose emitted word is architecturally unallocated), 4 parser-masked encoder acceptances proven at unit level but hidden behind earlier command-line parser rejections (#37 #52 #196 #198), and C-level #3 #510; full reconciliation in the cross-tabulation under 3.3 | 459 | 90.2% |
| **S3** | **Crash on malformed input** | panic; no wrong code shipped (9 panics reachable through the command-line interface #136 #234 #291 #379 #384 #436 #441 #456 #466, + #237 proven by unit test but hidden at the command-line level because the parser rejects first) | 10 | 2.0% |
| **S4** | **Missing or insufficient diagnostics** | encodes with a missing or wrong warning where the references diagnose (6 missing-warning CONSTRAINED UNPREDICTABLE issues #150 #163 #265 #320 #332 #497 + 5 issues where ccc warns but still accepts what the references reject #74 #80 #130 #131 #132, + #509 silent missing runtime) | 12 | 2.4% |
| **S5** | **Valid input rejected** | no wrong code; availability/compatibility loss (#36 #51 #55 #326 #411) | 5 | 1.0% |
| — | false positive (user-confirmed) | no defect; llvm-mc compatibility enhancement requests (#30 #119 #247) | 3 | 0.6% |
| **Total** | | | **509** | **100%**† |

† percentages rounded to 0.1; the column sums to 100.1% due to rounding.

Crash-family footnote: 5 of the S4 warn-but-still-accept rows also carry issue-reported, unit-test-proven latent
encoder panics (#74 #80 #130 #131 #132), so **15 crash-capable paths** exist in total (10 in S3 + 5 latent in S4).

Why not CWE: the previous roll-up mapped L2+L3+L4+L7 → CWE-20 (64.2%), L1 → CWE-628 (17.9%),
L5 → CWE-190/681 (10.2%), L6 → CWE-478, L8 → CWE-1023, L9 → CWE-754 — i.e. one generic bucket
absorbs ~76% of the tracker, all outcome information is lost (a CWE-20 row may crash, silently
miscompile, or merely miss a warning), and the "vulnerability" framing misstates the risk: this is a
**correctness and robustness failure of a compiler**, whose worst mode is shipping wrong bytes quietly.

### 3.3 Verdict-Class Distribution (raw behavior counts, all 509)

Verdict labels are the ones used in the `Verdict` column of `full_verification_tracker.md`. Note that 3.2 and 3.3 count on **different axes** and do not share row boundaries: 3.3 counts raw verdicts, while 3.2 groups verdicts by what the compiler ships — one severity class can merge several verdict classes (S2, S4), and one verdict class can split across severity classes (parser-masked, C-level specials). The cross-tabulation below reconciles the two tables cell by cell.

| Verdict class | Count | % of 509 |
|---|---|---|
| silent-accept (gcc rejects, ccc accepts) — including #507, whose accepted word is architecturally unallocated | 453 | 89.0% |
| differential (valid input, wrong encoding vs both references — including x86 #264 FS-segment prefix and the half-precision floating-point encoding family) | 17 | 3.3% |
| panic-on-invalid (panic reachable through the command-line interface) | 9 | 1.8% |
| warning-class CU (gas warns, ccc silent; CONSTRAINED UNPREDICTABLE forms) | 6 | 1.2% |
| parser-masked (encoder defect proven at unit level; the command-line parser rejects the input earlier, so the defect is not reachable through the command line) | 5 | 1.0% |
| warn-accepts (ccc emits a warning but then accepts input the references reject) | 5 | 1.0% |
| reverse (valid input rejected) | 5 | 1.0% |
| C-level / encoding specials (all real: #2 #3 #4 #508 #509* #510) — *#509 conditional on an absent i686 sysroot | 6 | 1.2% |
| **not-a-defect / false positive (user-confirmed)** | **3** | **0.6%** |
| **Total** | **509** | **100%**‡ |

‡ percentages rounded to 0.1; the column sums to 100.1% due to rounding (exact: 453/509 = 89.0%, not 88.8% — the old figure was computed before #507 was counted into silent-accept).

Mapping to the severity ladder (3.2) — cross-tabulation (rows = verdict classes of 3.3, columns = severity classes of 3.2; row and column totals reproduce both tables exactly):

| Verdict class (3.3) | S1 silent wrong code | S2 invalid input silently encoded | S3 crash | S4 deficient diagnostics | S5 valid input rejected | not-a-defect | Row total |
|---|---|---|---|---|---|---|---|
| silent-accept | — | **453** (incl. #507) | — | — | — | — | **453** |
| differential | **17** | — | — | — | — | — | **17** |
| panic-on-invalid | — | — | **9** | — | — | — | **9** |
| reverse | — | — | — | — | **5** | — | **5** |
| warning-class CU | — | — | — | **6** | — | — | **6** |
| parser-masked | — | 4 (#37 #52 #196 #198 — encoder accepts invalid input, proven at unit level) | 1 (#237 — unit-test panic) | — | — | — | **5** |
| warn-accepts | — | — | — | **5** | — | — | **5** |
| C-level / encoding specials | 3 (#2 #4 #508) | 2 (#3 #510) | — | 1 (#509) | — | — | **6** |
| not-a-defect / false positive | — | — | — | — | — | **3** | **3** |
| **Column total (= severity ladder 3.2)** | **20** | **459** | **10** | **12** | **5** | **3** | **509** |

### 3.4 Tracker Accuracy (full verification — no extrapolation)

- **506/509 real (99.4%) · 3 false positives (0.6%)** — every issue tested live
- The 3 false positives (#30, #119, #247 — BICS/EON/ORN "GNU immediate alias") are
  **user-confirmed**: ccc *and* gas 2.42 both reject; only llvm-mc accepts. The issues'
  "GNU alias" premise is invalid → reclassify as llvm-mc compatibility enhancement requests.
- Fidelity: 3 amendments (#17 wrap-target detail, #150 gas-claim + severity, #497 severity) — defects stand.

---

## 4. Tool Dimension (discovery vs verification are different jobs)

| Tool | Role | Issues filed | Issues adjudicated (verified live) | Own filings confirmed real | Own false positives |
|---|---|---|---|---|---|
| **FM-Agent (Hoare-style reasoning over property-based testing campaigns)** | discovery | 507 | — (not a verification tool) | 504 | 3 (#30 #119 #247) |
| **Contract-Based Differential Validation (LLM agent, this session)** | verification (+ side discovery) | 2 (#509 #510) | **509 (all)** | 2 | 0 |
| **Total** | | **509** | | **506** | **3** |

- The 3 false positives were *found by* the verification session but are *attributed to*
  FM-Agent's filings (single pattern: alias syntax accepted only by llvm-mc, which gas 2.42 also
  rejects — reference ambiguity, not detection hallucination).
- Beyond verdicts, the session produced 3 text amendments (#17, #150, #497) and
  2 reclassifications (#150 → missing-warning class, #509 → conditional on absent i686 sysroot).

The two tools are complementary: FM-Agent generates candidate defects; Contract-Based Differential Validation adjudicates
them against the GNU and LLVM reference toolchains. Only 2 issues came from the verification session
because discovery was not its objective.

## 5. Key Findings & Recommendations

1. **The compiler's defining defect is diagnostic silence, not crashes.** Silent emission (severity classes S1 + S2)
   = 479/506 real defects (94.7%): the 459 S2 defects encode invalid input that gcc/clang reject; the 20 S1 defects
   encode valid input wrongly where the references encode it correctly — in both cases bytes leave the compiler with
   zero diagnostics.
   The crash figure in this bullet is **contrast, not an addend**: only 15 of 506 defects can crash the compiler at all
   (10 in S3 + 5 latent panic paths inside S4; see the crash footnote in 3.2). That 15 is a cross-cutting count overlapping
   S4, so it is never added to 479. Full partition of the 506 real defects:
   **479 silent emission (S1+S2) + 15 crash-capable (all of S3 + 5 of S4) + 12 remainder (the other 7 S4
   diagnostics-only defects + the 5 S5 valid-input-rejected defects) = 506.**
   Every layer L1–L6 is the same disease — the encoder trusts its operands.
2. **Five root-cause families explain the assembler layers**: operand-count validation (L1), register
   class/width (L2), stack-pointer/zero-register slot-31 identity (L4), immediate masking (L5), shift-kind defaults (L6) —
   each mechanically fixable; the `get_gpr_checked` + `check_arity` helpers alone prevent ~60 issues.
3. **The half-precision floating-point encoding family (9 issues: #358 #361 #366 #409 #414 #475 #478 #481 #484)** is the most
   user-harmful S1 class: valid `__fp16` (half-precision) source silently encoded as single-precision instructions — wrong
   numerics, zero diagnostics. Single root cause in `src/backend/arm/assembler/encoder/fp_scalar.rs`
   (the floating-point-type field selection checks only whether the register name starts with 'd', so half-precision registers fall through to the single-precision encoding).
4. **CONSTRAINED UNPREDICTABLE handling divergence (#150 #163 #320 #332 #497)**: ccc matches gas's acceptance but not its
   warnings; llvm-mc rejects. Recommend adopting gas-style warnings.

**Recommended actions (priority order):**
1. Fix the half-precision floating-point type-field selection in `encoder/fp_scalar.rs` (closes the 9-issue S1 family)
2. Introduce `get_gpr_checked` + `check_arity`; refactor all encoders to use them (~60 issues prevented mechanically)
3. Add range checks before every bit-masked immediate — masking without prior validation causes 52 defects where out-of-range immediates silently wrap (L5)
4. Adopt gas-style warnings for the CONSTRAINED UNPREDICTABLE classes (STXP with status register also used as source; LDR with writeback where the transfer register equals the base register) for parity with both references
5. Reclassify #30 #119 #247 in the tracker as llvm-mc compatibility enhancement requests, not defects

---

*Report generated 2026-09-15 (full-run revision; the earlier random-sample intermediate with 100 issues
has been superseded and removed). All 38 verification reports: `pbt-out/verified_bug/`.
Full 509-row cross-check: `pbt-out/full_verification_tracker.md`.*
