# Round 41 close-out — continuous mining log

## Issues filed this round
#1145 long double unlinks on ARM64/RISC-V (tf3 runtime missing) — link-completeness
#1146 wave-6 type constraints ×6 (volatile fn, void* deref, incomplete member, void[], wide-init, folded dup case)
#1147 typedef-to-VLA SEGFAULT (crash class; VM type loses element count through alias)
#1148 VLA placement ×2 (struct member, block-scope static; extends #1138)
#1149 wave-8 declarator ×3 (array cast, self-containing struct, enum fwd object)

## Extension comments (no new issue — same root)
#1088 += empty #if expression + #undef non-identifier
#1143 += #elif with no enclosing #if

## Verified-correct this round (Design Caveats)
- Cross-target × levels: 150 + 120 cases (ARM64/RISC-V qemu, i686 -m32) all == gcc-x64
- Mixed bitfield/plain layout: byte-exact == clang (gcc differs only in UNSPECIFIED slack bits, C11 6.7.9p10) — NOT a bug
- i686 FP-struct interop (st(0) conventions): all match gcc -m32
- goto-out-of-VLA stack balance (1000 rounds), union bitfields, enum negative/large switch, [a...b] ranges, typeof, scalar braces, conversions (ties/2^24/trunc), char*-unsigned char* subtraction (both reject)
- (double)2^64 → u64 roundtrip diverges from gcc — UB (C11 6.3.1.4p1), not filed

## Full regression
15/15 resident property groups green (lowering×4, preproc×3, opt×2, switch, linker, stack×2, cross×2, li, pseudo)
