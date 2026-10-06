# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fp_sgnj, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw. Rust cargo tests executed encode_fp_sgnj (KAT + properties).

## This campaign

| Function | Source | Properties | Result |
|----------|--------|------------|--------|
| encode_fp_sgnj | float.rs:95 | 7 | 5 passing / 2 failing |

## Sweep (standard, 1/1)

coverage_gaps: no line-level data; C++ reporter marked encode_fp_sgnj NOT LINKED (reporter gap — tests are cargo lib tests, not those binaries). Manual audit of documented behaviors: 3-op, R-type, ABI, rs1=rs2, arity/GPR, extra, rm-fourth all have properties. Remaining gaps are the two filed bugs. Closed: tier round spent.

## Totals (FUNCTION_INDEX union)

| Metric | Value |
|--------|-------|
| Total source files indexed | 14 |
| Total functions | 351 |
| PBT candidates | 206 |
| This campaign target | encode_fp_sgnj (tested) |
