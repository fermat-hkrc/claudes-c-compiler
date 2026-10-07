# Round 34 — stack_layout deep PBT

## Bugs Found
(none)

## Properties
| Property | Cases | Status |
|----------|-------|--------|
| p_stack_deep_vs_gcc (deep frames, 16 mixed-type locals, long double chains) | 250 | **passing** |
| p_vla_alloca_vs_gcc (VLA + __builtin_alloca + memset/strlen) | 200 | **passing** |
