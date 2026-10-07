# Round 31 — ir/lowering deep PBT

## Bugs Found
### #1084 — f128_bytes_to_x87_bytes truncates 49 mantissa bits
- **Formal:** ∀ decimal long-double literal L: bytes_ccc(L) == bytes_gcc(L) (correctly-rounded x87-80). FALSIFIED — 7/8 samples exactly 1 ulp LOW; 0.1L and pi misprint at the 20th digit.
- Bug report: bug_reports/long_double_literal_1ulp_low.md
- Status: **filed #1084**; property p_global_init_bytes_vs_gcc excludes kind 13 pending fix.

### #1087 — compound literal nested-array rows drop elements 2..n
- **Formal:** ∀ compound literal `(T[n][m]){rows...}` at global scope: backing object == gcc's. FALSIFIED — `&(int[2][2]){{10,20},{30,40}}` emits [10,0,30,0].
- Bug report: bug_reports/compound_literal_nested_array.md
- Status: **filed #1087**; property p_compound_designated_statics_vs_gcc excludes 2-D compound pending fix.

## Properties (final status)
| Property | Cases | Status |
|----------|-------|--------|
| p_global_init_bytes_vs_gcc (14 type kinds) | 300 | **passing** (long double → #1084) |
| p_pointer_globals_vs_gcc (&g/&arr[k]/decay/member/ptr-arrays) | 400 | **passing** |
| p_complex_globals_vs_gcc (float/double _Complex) | 300 | **passing** |
| p_compound_designated_statics_vs_gcc (compound/designated/statics) | 400 | **passing** (2-D compound → #1087) |

## Design Caveats (verified-correct, not bugs)
- Regular (non-compound) 2-D global initializers: byte-exact — Doc evidence: round-31 property table above (kind 6 `int[4][3]` in p_global_init_bytes_vs_gcc, 300 cases green).
- Designated initializers out-of-order, anonymous unions, bitfield globals, static locals: all byte-exact vs gcc — Doc evidence: p_compound_designated_statics_vs_gcc case pool.
