# Extra-Operand Sweep: ARM data_processing — ALL DUPLICATES (2025-01 round 30)

Sweep result: 6 functions silently ignore extra operands. ALL already reported.
Old dedup DB was stale — refresh caught them.

| Function | Behavior | Duplicate of |
|----------|----------|-------------|
| encode_neg | 3rd operand ignored | #223 |
| encode_negs | 3rd operand ignored | #225 |
| encode_mvn | 3rd operand ignored | #215 |
| encode_mul | 4th operand ignored | #208 |
| encode_smull | 4th operand ignored | #276 |
| encode_madd | 5th operand ignored | #176 |

Teammate's recent wave (#1072 #1074 #1076 #1079 #1081) = same pattern on RISC-V functions.
Pattern fully covered. No new issue.

## SCVTF/UCVTF fixed-point silently misencoded → #360 (2025-01 round 30 cont.)

E2E confirmed: `scvtf d0,x1,#32` / `ucvtf d0,x1,#32` / `scvtf s0,w1,#16` all accepted
by assembler, #fbits silently dropped, encoded as 2-op integer form:
- objdump: `9e620020 scvtf d0, x1` (correct fixed-point: 0x9e428020)
- semantics diverge: x1=0x100000000 → int form 4294967296.0 vs fixed#32 1.0

Duplicate of **#360** "encode_int_to_float ignores extra operands" (body covers
fixed-point/fbits). Filed under helper name, not wrapper names — search bodies too.
Related family: #353 encode_fcvt_rounding extra operands, #361 H-dest, #362 SP/WSP, #363 GP-dest.

encode_prfop: all 18 prfop→u32 values verified correct (ARM ARM).
encode_bti: all 4 constants correct (BTI=HINT#32=0xd503241f, c/j/jc).

## RISC-V atomics non-zero offset silently dropped → #960/#962/#964 (round 30 cont.)

lr.w/lr.d/sc.w/amoswap.w/amoadd.d all accept `offset(rs1)` and drop offset silently.
E2E confirmed (.o emitted with `lr.w a0,(a1)` for source `lr.w a0, 4(a1)`).
llvm: "optional integer offset must be 0".
Filed under base names: encode_lr→#962, encode_sc→#964, encode_amo→#960 (+ extra-operand
#961/#963/#959, family #833). Lesson: check base-name variants of suffixed fns.

## RISC-V pseudo.rs sweep — 20 fns verified CORRECT (round 30 cont.)
- encode_li_immediate: 2048-case full-i64 execution-equivalence roundtrip ✓
  (4-case algorithm: addi / lui+addiw / shift / two-level shift)
- not/neg/negw/sext.w/seqz/snez/sltz/sgtz: byte-exact vs llvm-mc ✓
- csrr/csrw/csrs/csrc: byte-exact ✓
- fmv.s/d, fabs.s/d, fneg.s/d: byte-exact ✓ (mattr=+f,+d)
- mv: repo uses add rd,x0,rs2 (R-type) vs llvm canonical addi — valid alternative, not a bug
- bgt/ble/bgtu/bleu operand swap: verified correct
- get_branch_target Reg-as-symbol: documented design choice, not a bug
