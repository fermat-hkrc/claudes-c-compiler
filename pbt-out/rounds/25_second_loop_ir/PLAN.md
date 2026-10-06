# PBT Campaign: second full-repo loop, station 2 — ir + common (round 25)

## Findings
- mem2reg promotion conditions (C-observable): boundary sizes 7/8/9/16B,
  address-taken escapes (pointer arg + &x), volatile locals, branch-merge
  phi generation, loop-carried variable chains — 5/5 identical to gcc at
  -O0/-O2. Round-06's 13 unit properties remain green.
- common untested surface = spans/diagnostics/temp_files/type_builder
  (I/O glue, integration-bound) — skipped with reason; asm_constraints'
  single table predicate noted for a future micro-round.

## Module: second_loop_2
- [x] Scan/Test/Review — ir + common stations CLEAN; zero bugs
