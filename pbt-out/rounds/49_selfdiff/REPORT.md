# Round 49 — two NEW testing levels applied

## Level A: self-differential (oracle-free)
p_selfdiff_o0_o2 — 2000 rich semantic programs (structs/bitfields/unions/float/
switch/pointer chains), each compiled by ccc at -O0 AND -O2, both executed,
exit codes must match. ALL GREEN. The optimizer is self-consistent at scale;
this channel needs no gcc → 10x case budget available for regression use.

## Level B: valgrind runtime memory validation
Full-feature battery (nested struct-return chains, VLA+alloca mixing, cross-unit
bitfields, heap, _Complex arithmetic, builtin memcpy) at -O0 and -O2:
ZERO invalid reads/writes, zero leaks, at both levels. ccc-produced binaries
are valgrind-clean for valid programs.
(Initial GPF scare traced to MY invalid test program — pass3(struct)->double
chained — accepted by ccc per the ALREADY-FILED #1139 type-check family;
correctly-separated valid forms L1/L2/L3 all clean.)

## Levels NOT applied (out of tooling scope)
- Coverage-guided fuzzing (needs instrumented build; no .gcda infra)
- 10k+ TU scale runs (perf dimension, not correctness)
