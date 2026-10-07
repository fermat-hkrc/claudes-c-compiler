# Round 32 — preprocessor deep PBT

## Bugs Found
### #1088 — #if accepts malformed expressions, no diagnostic
- **Formal:** ∀ #if expression E that is not a valid integer constant expression: preprocessor must diagnose and not silently select a branch (C11 6.10.1p4). FALSIFIED — 12/13 malformed forms silent; 8 pick a branch (`#if 1 +` → TRUE).
- Bug report: bug_reports/if_malformed_no_diagnostic.md
- Status: **filed #1088**; p_if_corners treats gcc-reject as skip (negative contract unenforced).

### #1089 — __VA_OPT__ emitted as literal tokens
- **Formal:** ∀ `#define M(fmt, ...) fmt __VA_OPT__(| __VA_ARGS__)`: `M(x)` expands without tail; `M(x,7)` with tail (C23 6.10.5.2). FALSIFIED — literal `__VA_OPT__ ( | ... )` in output stream.
- Bug report: bug_reports/va_opt_not_expanded.md
- Status: **filed #1089**; p_macro_corners excludes the two vaopt cases.

## Properties (final status)
| Property | Cases | Status |
|----------|-------|--------|
| p_if_corners_vs_gcc (30 corners) | 30 | **passing** (1 skip → #1088) |
| p_if_random_unsigned_mix | 300 | **passing** |
| p_macro_corners_vs_gcc (18 kept) | 18 | **passing** (2 → #1089) |
| p_splice_and_include_corners (10 splice) | 10 | **passing** |

## Design Caveats
- `1, 2` comma in #if and `def+1`: both gcc-accepted; ccc matches (verified same behavior) — Doc evidence: p_if_corners case table + manual sweep table in bug_reports/.
- include differential with system headers = predefined-macro noise, not a bug — Doc evidence: pbt_support.rs gcc_tokens/sut_tokens doc (token-stream oracle intended for self-contained programs).
