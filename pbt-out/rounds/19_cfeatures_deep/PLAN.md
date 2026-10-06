# PBT Campaign: C-features deep + i686 assembler validation (round 19)

## Scan findings
- C-features E2E battery (16 programs vs gcc at -O0/-O2, natively run):
  struct-by-value (2-GP, 2-SSE, MIXED INTEGER+SSE eightbytes, 40B MEMORY
  class), varargs (incl. 5-arg), function pointers + tables, static/volatile,
  switch fallthrough, recursion (fib 20), unions w/ type-punning reads,
  long double 1/3*3 identity, __int128 shifts/mul/high-lo, VLA params,
  bitfield packing (5/11/16 bits), linked-list traversal — ALL match gcc.
  (One probe artifact: %.20Lg with a double arg is UB — both compilers print
  garbage, differently; the y==1.0L identity passed on both.)
- i686 assembler validation sweep: GPR width mixing silently accepted
  (movl %eax, %ax / mov %al, %bx / xorl %eax, %bx) — i686's own assembler
  path (#800); imm-range probes are GNU-compatible (no false positives).

## Module: cfeatures_deep
- [x] Scan (16-program battery + 17-form i686 probe matrix)
- [x] Plan (battery + PBT witness property)
- [x] Test (features 16/16 match gcc; p_i686_width_mix_rejected FAILING with shrunk witnesses)
- [x] Review (#800 filed, red regression committed)
