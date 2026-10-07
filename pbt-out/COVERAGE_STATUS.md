# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_vmv_v_v, English)
> Files: 16/16 scanned (100%) | Functions: 383 total | PBT candidates: 230 | This campaign tested: encode_vmv_v_v

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 230 |
| **This campaign target** | encode_vmv_v_v |
| **This campaign result** | 5 passing / 2 failing properties (2 bugs) |
| Coverage evidence | file-level (symbol presence) — no .gcda/.profraw; C++ reporter listed encode_vmv_v_v NOT LINKED; function executed under cargo test --lib encode_vmv_v_v |

## Module Breakdown

| Module | Scanned | Tested this campaign | Notes |
|--------|---------|----------------------|-------|
| encode_vmv_v_v | yes | yes | change surface; vector.rs:154 |

## Oracle Type Distribution

| Oracle Type | This campaign |
|-------------|---------------|
| differential | 1 passing |
| algebraic.invariant | 1 passing |
| algebraic.metamorphic | 2 passing |
| negative_error | 1 passing / 2 failing |

## File Coverage

| Source File | This campaign |
|-------------|---------------|
| vector.rs | encode_vmv_v_v tested |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> encode_vmv_v_v extra operand and trailing v0.t are silently ignored.

| Function | Source |
|----------|--------|
| encode_vmv_v_v | vector.rs |
