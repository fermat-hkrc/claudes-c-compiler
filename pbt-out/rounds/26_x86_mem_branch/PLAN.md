# PBT Campaign: x86 memory-operand/conversion + branch encodings + ARM/RISC-V FP (round 26)

## Findings
- #818: x86 assembler lacks the SSE conversion/shuffle subset — cvtt*2si*
  ENTIRELY (register and memory forms), cvtsi2*/pshuflw/pshufhw/pshufd
  MEMORY-source forms, maskmovq/maskmovdqu. All double-reference confirmed
  (as ∧ llvm-mc accept, ccc rejects). Same class as #777/#782.
- Clean surfaces: x86 branch encodings (16 jcc + call/jmp + loop: bytes
  identical to as); ARM conversions + FP/vector loads/stores (scvtf/fcvtzs/
  fmov/ldr-str s,d,q: 15/15 byte-exact vs llvm-mc); RISC-V FP conversions
  (fcvt/fmv/fsgnj/fle/flt/feq: 14/14 byte-exact); i686 -S roundtrip MATCH
  (no #807-class bloat on 32-bit); lock-prefixed and rep-prefixed ops OK.

## Module: x86_mem_branch
- [x] Scan (24+16+15+14 form probe matrices, byte-level where accepted)
- [x] Plan / [x] Test / [x] Review — #818 filed, red regression committed
