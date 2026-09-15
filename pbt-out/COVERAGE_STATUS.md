# COVERAGE STATUS

Campaign: claudes-c-compiler (ccc), x86-64, differential vs gcc-13.3.
Canonical per-function ledger: pbt-out/COVERAGE.md (append-only).

- Scanned: src/common/{encoding,const_arith,types}.rs — 43 functions indexed
  (FUNCTION_INDEX.md), 8 PBT candidates; plus full-pipeline E2E surface.
- Tested: 4 unit targets (encoding P6/P7, const_arith P8, types P9) + E2E pipeline
  (P1-P5, 1000 generated programs each) — final verdicts in REPORT.md.
- Skipped modules & reasons: PLAN.md `Skipped modules`.
- Native coverage: pbt-out/code-coverage/ (harness-rendered).
- coverage_gaps sweep (standard tier, 1 round): see REPORT.md.
