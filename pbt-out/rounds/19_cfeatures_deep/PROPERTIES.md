# PROPERTIES.md — round 19

## p_i686_width_mix_rejected
- Status: failing — shrunk counterexample `movl %ax, %eax`-class; every
  width-mixed direction silently accepted by ccc -m32 while as --32 AND
  llvm-mc -triple=i386 reject → issue #800.
- Test file: src/backend/i686/mod.rs (pbt_witness)

## C-features E2E battery (probe-grade, 16 programs)
- Status: all match gcc at -O0 and -O2 (struct classification incl. SSE/MIXED
  eightbytes, varargs, fn-ptrs, static/volatile, switch, recursion, unions,
  long double identity, __int128, VLA, bitfields, linked lists).
