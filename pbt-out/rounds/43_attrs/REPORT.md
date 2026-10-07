# Round 43 — attribute/builtin/extension semantics battery

## Bugs Found
### #1153 — constructor(N) priorities ignored
- Formal: ∀ ctors p1<p2: p1 first; default (no-N) last (gcc .init_array.NNNN semantics). Witness: gcc 2→101→default; ccc declaration order. Reproduce: /tmp/cd.c.
### #1154 — #if operand-type constraints (5 forms)
- Formal: #if expr = integer constant expr, NO casts (6.10.1p4), no float/string/pp-number. Witness: 1.0/1e5/"x"/(int)'A'/0x1e+2 all accepted. Reproduce: /tmp/pp43.sh.

## Verified-correct (green)
- __builtin bit family at boundaries: bswap32/64, clz/ctz/popcount/parity over {0,1,0x80000000,MAX} (only UB clz(0) diverges — test artifact)
- _Complex runtime arithmetic at -O2: add/mul/DIV (Smith), float-complex div, conj, cabs — exact vs gcc
- asm volatile("":::"memory") compiler barrier at -O2: both loads AND both stores preserved (disasm witness = gcc shape)
- _Alignas on struct MEMBER (MA=32 off=16), cleanup attribute (runs on scope exit), _Thread_local, section attribute
