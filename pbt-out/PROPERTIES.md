# Properties: encode_neon_shrn

## encode_neon_shrn_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SHRN decoder). Sibling encode_neon_qshrn / encode_neon_sqshrun / encode_neon_scalar_qshrn / encode_neon_three_diff_narrow rejected by same-job gate (saturating / scalar / ADDHN, different opcodes). README.md:12 claims gas-compatible textual assembly; llvm-mc -triple=aarch64 is the independent reference with verified mapping.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: src/backend/arm/assembler/encoder/neon.rs:4744 encode_neon_qshrn_pbt (narrowing Ta/Tb pairing); encode_neon_sri_pbt.rs llvm-mc differential shape
- Formal: ∀ rd,rn ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high ∈ {0,1}, opcode ∈ {100001,100011}, shift ∈ [1, dest_esize(Ta)]. encode_neon_shrn([Vd.Tb, Vn.Ta, #shift], opcode, is_high) = llvm-mc("{mnem} Vd.Tb, Vn.Ta, #shift") where Tb = mandated(Ta, is_high) and mnem is shrn/shrn2/rshrn/rshrn2
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, opcode, shift]
  domain: { rd: 0..31, rn: 0..31, ta: {8h,4s,2d}, is_high: bool, opcode: {0b100001,0b100011}, shift: 1..dest_esize(ta) }
  relation:
    op: eq
    lhs: encode_neon_shrn([RegArrangement(Vd,Tb), RegArrangement(Vn,Ta), Imm(shift)], opcode, is_high)
    rhs: llvm_mc("{mnem} Vd.Tb, Vn.Ta, #shift")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  opcode: { gen: oneof, options: [33, 35] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: README.md:12 assembler accepts the same textual assembly that GCC gas would consume; encoder/mod.rs:648-651 dispatch; ARM Advanced SIMD SHRN/RSHRN
```

## encode_neon_shrn_metamorphic_rd_rn_q
- Tier: 4
- Rationale: Algebraic metamorphic over ARM field isolation: changing only Rd/Rn/Q/opcode must affect only the corresponding bits. Stronger differential already used on the valid domain; this property does not need llvm-mc and isolates bit-field bugs the word-equality check can miss in a single sample. Rejected round-trip (no decoder). Evidence: neon.rs:1434 format comment plus ARM SHRN layout.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_metamorphic_rd_rn_q
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ [0,31]. let w(rd,rn,q,op) = encode_neon_shrn(Vd.8b, Vn.8h, #1, op, q). (w(rd1,rn1,0,SHRN) ⊕ w(rd2,rn1,0,SHRN)) & ~0x1F = 0 ∧ (w(rd1,rn1,0,SHRN) ⊕ w(rd1,rn2,0,SHRN)) & ~(0x1F<<5) = 0 ∧ (w(rd1,rn1,0,SHRN) ⊕ w(rd1,rn1,1,SHRN)) & ~(1<<30) = 0 ∧ (w(rd1,rn1,0,SHRN) ⊕ w(rd1,rn1,0,RSHRN)) & ~(0x3F<<10) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: 0..31, rd2: 0..31, rn1: 0..31, rn2: 0..31 }
  body: changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only is_high in bit 30; only SHRN vs RSHRN opcode in bits[15:10]
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1434 Format 0 Q 0 01111 0 immh immb opcode 1 Rn Rd; ARM SHRN Q at bit 30, Rn[9:5], Rd[4:0], opcode[15:10]
```

## encode_neon_shrn_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM SHRN/RSHRN word: bit31=0, Q=is_high, U=0, bits[28:23]=011110, immh:immb=source_esize-shift, bits[15:10]=opcode, Rn, Rd. Weaker than differential (does not check agreement with an independent assembler) but pins the documented format comment independently of llvm-mc.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_invariant_arm_fields
- Formal: ∀ rd,rn ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high ∈ {0,1}, opcode ∈ {100001,100011}, shift ∈ [1, dest_esize(Ta)]. let w = encode_neon_shrn(...). w[31]=0 ∧ w[30]=is_high ∧ w[29]=0 ∧ w[28:23]=011110 ∧ w[22:16]=source_esize(Ta)-shift ∧ w[15:10]=opcode ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, opcode, shift]
  domain: { rd: 0..31, rn: 0..31, ta: {8h,4s,2d}, is_high: bool, opcode: {0b100001,0b100011}, shift: 1..dest_esize(ta) }
  body: ARM SHRN/RSHRN field layout as documented at neon.rs:1434
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  opcode: { gen: oneof, options: [33, 35] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: neon.rs:1434 Format 0 Q 0 01111 0 immh immb opcode 1 Rn Rd; neon.rs:1435 SHRN opcode=10000, RSHRN opcode=10001; ARM immh:immb = source_esize - shift
```

## encode_neon_shrn_neg_arity
- Tier: 5
- Rationale: Negative/error contract: fewer than 3 operands must Err. Evidence: neon.rs:1437 `if operands.len() < 3 { return Err("shrn/rshrn requires 3 operands") }` plus llvm-mc/gas reject arity 0–2. Documented by the function's own arity check and by the assembler gas-compatibility claim.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn ∈ [0,31]. encode_neon_shrn(ops[0..n], 0b100001, false) = Err ∧ llvm-mc(arity-n shrn) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn]
  domain: { n: 0..2, rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    lhs: encode_neon_shrn(ops.take(n), 0b100001, false)
    rhs: Err
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1437 shrn/rshrn requires 3 operands; llvm-mc rejects arity 0-2; README.md:12 gas compatibility
```

## encode_neon_shrn_neg_extra_operand
- Tier: 5
- Rationale: Negative/error contract: a fourth operand must Err. llvm-mc and gas reject `shrn Vd.Tb, Vn.Ta, #shift, extra`. README.md:12 gas compatibility. The SUT only checks `len < 3` (extra ignored) — that is the contract under test, not a reason to drop the input. Domain is the documented 3-operand SHRN form plus one extra register; extra is not declared invalid by an input-domain restriction on this function (the arity comment only names the missing-operand case).
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high ∈ {0,1}, opcode ∈ {100001,100011}, shift ∈ [1, dest_esize(Ta)]. encode_neon_shrn([Vd.Tb, Vn.Ta, #shift, Vextra.Tb], opcode, is_high) = Err ∧ llvm-mc(4-operand) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_shrn([v0.8b, v0.8h, #1, v0.8b], opcode=0b100001, is_high=false) → Ok(Word)
- Bug report: pbt-out/bug_reports/encode_neon_shrn_extra_operand.md

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ta, is_high, opcode, shift]
  domain: { rd: 0..31, rn: 0..31, extra: 0..31, ta: {8h,4s,2d}, is_high: bool, opcode: {0b100001,0b100011}, shift: 1..dest_esize(ta) }
  relation:
    op: throws
    lhs: encode_neon_shrn([Vd.Tb, Vn.Ta, Imm(shift), Vextra.Tb], opcode, is_high)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  opcode: { gen: oneof, options: [33, 35] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
expected_error: String
evidence: README.md:12 gas compatibility; llvm-mc rejects a fourth operand on shrn/rshrn; ARM SHRN is a 3-operand instruction
```

## encode_neon_shrn_neg_invalid_arrangement
- Tier: 5
- Rationale: Negative/error contract: ARM SHRN requires Ta in {8H,4S,2D} and Tb the matching narrow arrangement (8B/16B, 4H/8H, 2S/4S) for Q. llvm-mc rejects mismatched/reserved arrangements. Dest arrangement is discarded by the SUT (`let (rd, _)`) — keep dest in the domain; a silent encode of an invalid pair is the finding, not an exclusion.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_neg_invalid_t; encode_neon_qshrn_pbt dest Tb check
- Formal: ∀ rd,rn ∈ [0,31], Tb,Ta arrangements, shift ∈ [1,64], is_high, opcode. (Tb,Ta,is_high) not a valid SHRN pair ⇒ encode_neon_shrn = Err ∧ llvm-mc = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_shrn([v0.8b, v0.2d, #1], opcode=0b100001, is_high=false) → Ok(Word)
- Bug report: pbt-out/bug_reports/encode_neon_shrn_mismatched_dest_tb.md

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, shift, is_high, opcode]
  domain: { rd: 0..31, rn: 0..31, tb: arrangements, ta: arrangements, shift: 1..64, is_high: bool, opcode: {0b100001,0b100011} }
  relation:
    op: throws
    lhs: encode_neon_shrn([Vd.Tb, Vn.Ta, Imm(shift)], opcode, is_high)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  ta: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  is_high: { gen: bool }
  opcode: { gen: oneof, options: [33, 35] }
expected_error: String
evidence: ARM Advanced SIMD SHRN Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects mismatched pairs; README.md:12 gas compatibility
```

## encode_neon_shrn_neg_shift_oob
- Tier: 5
- Rationale: Negative/error contract: ARM SHRN shift must be in [1, dest_esize]. llvm-mc rejects #0 and dest_esize+1. neon.rs:1444 checks `shift == 0 || shift > half_bits`. Bounds 0, dest_esize+1, -1, and dest_esize (in-range) must be sampled exactly. Dest_esize is 8/16/32 for Ta 8h/4s/2d.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_neg_shift_oob; encode_neon_qshrn_pbt shift range (qshrn incorrectly uses source esize)
- Formal: ∀ rd,rn ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high, opcode, shift ∉ [1, dest_esize(Ta)]. encode_neon_shrn = Err ∧ (shift in a llvm-mc-representable range ⇒ llvm-mc = Err)
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_shrn([v0.8b, v0.8h, #4294967297], opcode=0b100001, is_high=false) → Ok(Word) of shift #1
- Bug report: pbt-out/bug_reports/encode_neon_shrn_shift_i64_trunc.md

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, opcode, shift]
  domain: { rd: 0..31, rn: 0..31, ta: {8h,4s,2d}, is_high: bool, opcode: {0b100001,0b100011}, shift: {0, dest_esize+1, -1, dest_esize*2, i64::MIN} }
  relation:
    op: throws
    lhs: encode_neon_shrn([Vd.Tb, Vn.Ta, Imm(shift)], opcode, is_high)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  opcode: { gen: oneof, options: [33, 35] }
  shift: { gen: oneof, options: [0, -1, 9, 17, 33, 65, -1] }
expected_error: String
evidence: ARM SHRN shift in [1, dest_esize]; llvm-mc "immediate must be an integer in range [1, 8]" for .8b; neon.rs:1444 shift == 0 || shift > half_bits
```

## encode_neon_shrn_neg_gpr_or_bare
- Tier: 5
- Rationale: Negative/error contract: dest/src must be NEON RegArrangement (Vd.Tb, Vn.Ta). llvm-mc/gas reject bare V, GPR x/w, and scalar d/s prefixes. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/v/h/b — keep those inputs; a silent encode as a V register is the finding.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ [0,31], kind ∈ {bare dest, bare src, GPR dest arrangement, GPR src, scalar dest}. encode_neon_shrn(kind) = Err ∧ llvm-mc(kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_shrn([Reg("v0"), v0.8h, #1], opcode=0b100001, is_high=false) → Ok(Word)
- Bug report: pbt-out/bug_reports/encode_neon_shrn_bare_dest.md

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind]
  domain: { rd: 0..31, rn: 0..31, kind: {bare_dest, bare_src, gpr_dest_arr, gpr_src, scalar_dest} }
  relation:
    op: throws
    lhs: encode_neon_shrn(kind_ops, 0b100001, false)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: README.md:12 gas compatibility; llvm-mc rejects `shrn v0, v1.8h, #1` and `shrn x0.8b, v1.8h, #1`; ARM SHRN operands are SIMD vector registers with arrangements
```

## encode_neon_shrn_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — uppercase mnemonic and V prefix must agree with llvm-mc (parse_reg_num lowercases). Same differential oracle as the valid-domain property. Added after coverage_gaps reported no profraw; manual arm audit of parse_reg_num / arrangement path.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_diff_alt_spellings
- Formal: ∀ rd,rn ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high, opcode, shift ∈ [1, dest_esize(Ta)]. encode_neon_shrn([V{rd}.Tb, V{rn}.Ta, #shift], opcode, is_high) = llvm-mc("{MNEM} Vd.TB, Vn.TA, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, opcode, shift]
  domain: { rd: 0..31, rn: 0..31, ta: {8h,4s,2d}, is_high: bool, opcode: {0b100001,0b100011}, shift: 1..dest_esize(ta) }
  relation:
    op: eq
    lhs: encode_neon_shrn([RegArrangement(Vrd,Tb), RegArrangement(Vrn,Ta), Imm(shift)], opcode, is_high)
    rhs: llvm_mc("{MNEM} Vd.TB, Vn.TA, #shift")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:177 parse_reg_num lowercases; README.md:12 gas compatibility
```

## encode_neon_shrn_neg_nonreg
- Tier: 5
- Rationale: Sweep — Imm/Mem/Label at dest, source, or shift slot must Err. Hits get_neon_reg `other` arm and get_imm error path that the first batch did not drive. llvm-mc rejects a memory/immediate dest.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: encode_neon_sri_pbt.rs encode_neon_sri_neg_nonreg
- Formal: ∀ rd,rn ∈ [0,31], kind ∈ {Imm, Mem, Label}, slot ∈ {0,1,2}. encode_neon_shrn(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: 0..31, rn: 0..31, kind: {Imm, Mem, Label}, slot: 0..2 }
  relation:
    op: throws
    lhs: encode_neon_shrn(ops_with_slot_replaced, 0b100001, false)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: neon.rs:7-19 get_neon_reg other arm; encoder/mod.rs:1014 get_imm expects Imm; README.md:12 gas compatibility
```

## encode_neon_shrn_neg_unsupported_source
- Tier: 5
- Rationale: Sweep — ARM SHRN Ta is only {8H,4S,2D}. The first-batch invalid-arrangement property shrank immediately to a dest-mismatch (successes: 0), so the match `_` arm at neon.rs:1442 was not shown to run. Dedicated generator over invalid Ta.
- Doc contract: neon.rs:1434 "Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd" — asserted fingerprint bf443590
- Seed: neon.rs:1441-1442 match arr_n unsupported source
- Formal: ∀ rd,rn ∈ [0,31], Ta ∈ {8b,16b,4h,2s,1d,1q}, is_high, opcode. encode_neon_shrn([Vd.Tb, Vn.Ta, #1], opcode, is_high) = Err ∧ llvm-mc = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_shrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, opcode]
  domain: { rd: 0..31, rn: 0..31, ta: {8b,16b,4h,2s,1d,1q}, is_high: bool, opcode: {0b100001,0b100011} }
  relation:
    op: throws
    lhs: encode_neon_shrn([Vd.Tb, Vn.Ta, Imm(1)], opcode, is_high)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8b", "16b", "4h", "2s", "1d", "1q"] }
expected_error: String
evidence: neon.rs:1441-1442 shrn: unsupported source; ARM SHRN Ta in {8H,4S,2D}
```
