# PBT Campaign: exotic-C deep battery (round 21)

## Scan findings
- 11-feature battery vs gcc (native execution, -O0/-O2): C11 atomics
  (fetch_add/exchange/cmpxchg), setjmp/longjmp (+ volatile-clobber classic),
  computed goto (labels-as-values dispatch), alloca + VLA matrix (runtime
  dims), long double arithmetic (x87), signed/unsigned bitfield wraparound,
  sparse switch (jump-table shapes), va_copy, __builtin_* family
  (bswap/clz/ctz/popcount/constant_p/expect), _Complex double+float
  (arithmetic + cabs + conjugate), pthreads — 10/11 match gcc EXACTLY.
- 1 documented-feature gap → #805: x86 tail-call optimization misses the
  `return f(...)` shape whose result joins a shared epilogue label
  (tail_call.rs:143 requires the epilogue INLINE within 30 lines; any
  intervening Jump kind bails). 1M-deep tail recursion segfaults under
  ccc -O2 (gcc -O2 converts to a loop). asm-confirmed.
- Note (not filed): -O0 frame bloat (48B + 5 callee-saves for a 2-local
  function halves recursion depth vs gcc -O0) — already tracked in
  ideas/reduce_stack_frame_size*.txt.

## Module: exotic_deep
- [x] Scan (11 features + depth-bisect + asm analysis)
- [x] Plan / [x] Test / [x] Review — #805 filed (dedup-clean)
