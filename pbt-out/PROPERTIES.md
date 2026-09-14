# Properties: encode_neon_shll

## encode_neon_shll_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler of the same GNU-style SSHLL/USHLL text). State machine rejected: single encoding call, no lifecycle. Round-trip rejected: no in-tree SSHLL decoder. encode_neon_shl / encode_neon_shift_left_imm / two-misc SHLL fail the same-job gate (same-width or shift-by-esize). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping [RegArrangement(Vd,Ta), RegArrangement(Vn,Tb), Imm(shift)] + (u_bit, is_high) <-> `{sshll|ushll}{2?} Vd.Ta, Vn.Tb, #shift`.
- Seed: src/backend/arm/assembler/README.md:230 NEON widen/long table
- Formal: ∀ rd,rn ∈ 0..31, ∀ (Tb,Ta,esize,is_high) ∈ {(8b,8h,8,false),(16b,8h,8,true),(4h,4s,16,false),(8h,4s,16,true),(2s,2d,32,false),(4s,2d,32,true)}, ∀ shift ∈ 0..(esize-1), ∀ u_bit ∈ {0,1}. encode_neon_shll([Vd.Ta, Vn.Tb, Imm(shift)], u_bit, is_high) = llvm-mc(`{s|u}shll{2?} Vd.Ta, Vn.Tb, #shift`)
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shll
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, shift, u_bit, is_high]
  domain: { rd: v0..v31, rn: v0..v31, (tb,ta,is_high): mandated_widen_pair, shift: 0..(esize(tb)-1), u_bit: 0..1 }
  relation:
    op: eq
    lhs: encode_neon_shll([RegArrangement(rd,ta), RegArrangement(rn,tb), Imm(shift)], u_bit, is_high)
    rhs: llvm_mc_word("{s|u}shll{2?} Vd.Ta, Vn.Tb, #shift")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
evidence: "README.md:1-14 gas-compatible AArch64 assembler; README.md:230 sshll/ushll; encoder/mod.rs:614-617 dispatch; ARM ARM Advanced SIMD SSHLL/USHLL; llvm-mc -triple=aarch64"
```

## encode_neon_shll_metamorphic_q
- Tier: 4
- Rationale: ARM ARM Q is bit 30 and selects the low vs high half (SSHLL vs SSHLL2). Stronger differential already covers valid encodings; this metamorphic isolates the Q bit. State machine / round-trip rejected as above.
- Seed: neon.rs:1470 Format 0 Q U 011110 ...; ARM ARM Q field
- Formal: ∀ valid (rd,rn,Tb,shift,u_bit) with low-half Tb. encode_neon_shll(ops, u_bit, true) XOR encode_neon_shll(ops, u_bit, false) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shll
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, tb, shift, u_bit]
  domain: { rd: v0..v31, rn: v0..v31, tb: {8b,4h,2s}, shift: 0..(esize-1), u_bit: 0..1 }
  relation:
    op: eq
    lhs: encode_neon_shll(ops, u_bit, true) XOR encode_neon_shll(ops, u_bit, false)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 4h, 2s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
evidence: "neon.rs:1470 Format 0 Q U 011110; ARM ARM SSHLL Q=0 vs SSHLL2 Q=1"
```

## encode_neon_shll_metamorphic_u
- Tier: 4
- Rationale: ARM ARM U is bit 29 and selects signed (SSHLL) vs unsigned (USHLL). Stronger differential already covers valid encodings; this metamorphic isolates the U bit.
- Seed: neon.rs:1470 Format 0 Q U 011110; encoder/mod.rs:614-617 u_bit 0/1
- Formal: ∀ valid (rd,rn,Tb,Ta,shift,is_high). encode_neon_shll(ops, 1, is_high) XOR encode_neon_shll(ops, 0, is_high) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shll
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, tb, shift, is_high]
  domain: { rd: v0..v31, rn: v0..v31, (tb,is_high): mandated_tb, shift: 0..(esize-1) }
  relation:
    op: eq
    lhs: encode_neon_shll(ops, 1, is_high) XOR encode_neon_shll(ops, 0, is_high)
    rhs: 1 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  is_high: { gen: bool }
evidence: "neon.rs:1470 U at bit 29; encoder/mod.rs:614-617 ushll u_bit=1 vs sshll u_bit=0; ARM ARM U field"
```

## encode_neon_shll_alias_xtl
- Tier: 2
- Rationale: neon.rs:160-161 documents UXTL/SXTL as aliases for USHLL/SSHLL with shift #0 (same job). llvm-mc prints sxtl as sshll #0. Independent sibling encode_neon_xtl is a differential companion on the shift=0 slice. Same-job evidence: comment on encode_neon_xtl plus llvm-mc alias expansion.
- Seed: neon.rs:160-161 "These are aliases for USHLL/SSHLL with shift #0"
- Formal: ∀ rd,rn ∈ 0..31, ∀ valid (Tb,Ta,is_high), ∀ u_bit ∈ {0,1}. encode_neon_shll([Vd.Ta, Vn.Tb, Imm(0)], u_bit, is_high) = encode_neon_xtl([Vd.Ta, Vn.Tb], u_bit, is_high) = llvm-mc(`{s|u}xtl{2?} Vd.Ta, Vn.Tb`)
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shll
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, u_bit, is_high]
  domain: { rd: v0..v31, rn: v0..v31, (tb,ta,is_high): mandated_widen_pair, u_bit: 0..1 }
  relation:
    op: eq
    lhs: encode_neon_shll([RegArrangement(rd,ta), RegArrangement(rn,tb), Imm(0)], u_bit, is_high)
    rhs: encode_neon_xtl([RegArrangement(rd,ta), RegArrangement(rn,tb)], u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
evidence: "neon.rs:160-161 UXTL/SXTL aliases for USHLL/SSHLL #0; encoder/mod.rs:844-848 uxtl/sxtl dispatch; llvm-mc expands sxtl to sshll #0"
```

## encode_neon_shll_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM / neon.rs:1470 document the exact bit layout. Weaker than differential; kept as a structural invariant over the success path. Q from is_high, U from u_bit, immh:immb = esize+shift, opcode=101001.
- Seed: neon.rs:1470 Format 0 Q U 011110 immh immb 10100 1 Rn Rd
- Formal: ∀ valid encoding word w. bit31=0 ∧ Q=is_high ∧ U=u_bit ∧ bits[28:23]=011110 ∧ immh:immb=esize+shift ∧ bits[15:10]=101001 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shll
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, tb, shift, u_bit, is_high]
  domain: { rd: v0..v31, rn: v0..v31, tb: mandated_tb, shift: 0..(esize-1), u_bit: 0..1, is_high: bool }
  relation:
    op: holds
    expr: word_matches_arm_sshll_layout(w, rd, rn, esize, shift, u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
evidence: "neon.rs:1470-1484 Format 0 Q U 011110 immh immb 101001 Rn Rd; ARM ARM Advanced SIMD SSHLL/USHLL"
```

## encode_neon_shll_neg_shift_oob
- Tier: 4
- Rationale: ARM ARM / llvm-mc require shift in [0, esize-1]. llvm-mc: "immediate must be an integer in range [0, 7]" for 8b (and analogously 0..15 / 0..31). Negative-error contract: out-of-range shift must Err. Documented bounds 0 and esize-1 are in the valid generator; this property pins -1, esize, esize+1.
- Seed: llvm-mc error on `sshll v0.8h, v1.8b, #8`; ARM ARM shift 0 to esize-1
- Formal: ∀ rd,rn, valid (Tb,Ta,is_high), u_bit, ∀ shift ∉ [0, esize(Tb)-1]. encode_neon_shll([Vd.Ta, Vn.Tb, Imm(shift)], u_bit, is_high) is Err ∧ llvm-mc rejects the asm
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, tb=8b, shift=-1, u_bit=0 (debug overflow on base_val + (shift as u32); also shift=8 encodes as a different esize)
- Bug report: pbt-out/bug_reports/encode_neon_shll_shift_oob.md

```property
function: encode_neon_shll
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, shift, u_bit, is_high]
  domain: { rd: v0..v31, rn: v0..v31, tb: mandated_tb, ta: mandated_ta, shift: oob_shift, u_bit: 0..1, is_high: bool }
  relation:
    op: throws
    expr: encode_neon_shll([RegArrangement(rd,ta), RegArrangement(rn,tb), Imm(shift)], u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: -16, max: 256, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: "ARM ARM SSHLL shift in 0..(esize-1); llvm-mc rejects sshll v0.8h, v1.8b, #8 as immediate out of range [0, 7]"
```

## encode_neon_shll_neg_dest_tb
- Tier: 4
- Rationale: ARM ARM mandates dest Ta from Q and source Tb (8B→8H, 16B→8H, 4H→4S, 8H→4S, 2S→2D, 4S→2D). llvm-mc rejects mismatched dest (e.g. `sshll v0.8b, v1.8b, #0`). Negative-error: mismatched dest arrangement must Err.
- Seed: llvm-mc error on `sshll v0.8b, v1.8b, #0`
- Formal: ∀ rd,rn, valid (Tb,shift,u_bit,is_high), ∀ Ta ≠ mandated_ta(Tb,is_high). encode_neon_shll([Vd.Ta, Vn.Tb, Imm(shift)], u_bit, is_high) is Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, tb=8b, ta=8b, shift=0, u_bit=0 (`sshll v0.8b, v0.8b, #0`)
- Bug report: pbt-out/bug_reports/encode_neon_shll_mismatched_dest_ta.md

```property
function: encode_neon_shll
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, shift, u_bit, is_high]
  domain: { rd: v0..v31, rn: v0..v31, tb: mandated_tb, ta: mismatched_ta, shift: 0..(esize-1), u_bit: 0..1, is_high: bool }
  relation:
    op: throws
    expr: encode_neon_shll([RegArrangement(rd,ta), RegArrangement(rn,tb), Imm(shift)], u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  ta: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s, 1d, 2d] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: "ARM ARM SSHLL Ta determined by Q:immh; llvm-mc rejects sshll v0.8b, v1.8b, #0 as invalid operand"
```

## encode_neon_shll_neg_extra_operand
- Tier: 4
- Rationale: GNU/llvm-mc SSHLL is a 3-operand instruction. llvm-mc rejects a 4th operand. Body checks `operands.len() < 3` only, so extra operands are a documented-error-path candidate. Negative-error: arity != 3 must Err.
- Seed: llvm-mc error on `sshll v0.8h, v1.8b, #0, v2.8h`
- Formal: ∀ valid 3-operand SSHLL ops, ∀ extra operand. encode_neon_shll(ops ++ [extra], u_bit, is_high) is Err ∧ llvm-mc rejects 4-operand asm
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, tb=8b, shift=0, u_bit=0 (`sshll v0.8h, v0.8b, #0, v0.8h`)
- Bug report: pbt-out/bug_reports/encode_neon_shll_extra_operand.md

```property
function: encode_neon_shll
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, tb, shift, u_bit, is_high]
  domain: { rd: v0..v31, rn: v0..v31, extra: v0..v31, tb: mandated_tb, shift: 0..(esize-1), u_bit: 0..1, is_high: bool }
  relation:
    op: throws
    expr: encode_neon_shll([Vd.Ta, Vn.Tb, Imm(shift), Vextra.Ta], u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: "README.md:1-14 gas-compatible; llvm-mc rejects sshll v0.8h, v1.8b, #0, v2.8h as invalid operand"
```

## encode_neon_shll_neg_gpr_dest
- Tier: 4
- Rationale: ARM ARM / llvm-mc require Vd.Ta (a NEON arrangement register). GPR/scalar FP dest (`x0`, `w0`, `d0`, ...) is invalid. get_neon_reg accepts Operand::Reg and dest arrangement is discarded, so this is a documented-error-path candidate. Coverage-sweep addition after dest arrangement was observed unused.
- Seed: llvm-mc error on `sshll x0, v1.8b, #0`
- Formal: ∀ prefix ∈ {x,w,d,s,q,h,b}, n ∈ 0..31, valid (Tb,shift,u_bit,is_high). encode_neon_shll([Reg(prefix n), Vn.Tb, Imm(shift)], u_bit, is_high) is Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: prefix=x, n=0, rn=0, tb=8b, shift=0, u_bit=0 (`sshll x0, v0.8b, #0`)
- Bug report: pbt-out/bug_reports/encode_neon_shll_gpr_dest.md

```property
function: encode_neon_shll
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prefix, n, rn, tb, shift, u_bit, is_high]
  domain: { prefix: xwdsqhb, n: 0..31, rn: v0..v31, tb: mandated_tb, shift: 0..(esize-1), u_bit: 0..1, is_high: bool }
  relation:
    op: throws
    expr: encode_neon_shll([Reg(prefix n), RegArrangement(rn,tb), Imm(shift)], u_bit, is_high)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: "ARM ARM SSHLL Vd.Ta; llvm-mc rejects sshll x0, v1.8b, #0 as invalid operand"
```

## encode_neon_shll_neg_src_vs_high
- Tier: 4
- Rationale: ARM ARM Q selects low vs high half of the source: SSHLL Tb in {8B,4H,2S}, SSHLL2 Tb in {16B,8H,4S}. llvm-mc rejects `sshll2 v0.8h, v0.8b, #0` and `sshll v0.8h, v0.16b, #0`. Coverage-sweep: Q comes only from is_high, not from Tb.
- Seed: llvm-mc error on `sshll v0.8h, v1.16b, #0`
- Formal: ∀ rd,rn, Tb, shift, u_bit. encode_neon_shll(ops, u_bit, !tb_is_high(Tb)) is Err ∧ llvm-mc rejects the mismatched 2-suffix
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, tb=8b, shift=0, u_bit=0, is_high=true (`sshll2 v0.8h, v0.8b, #0`)
- Bug report: pbt-out/bug_reports/encode_neon_shll_src_vs_high.md

```property
function: encode_neon_shll
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, tb, shift, u_bit]
  domain: { rd: v0..v31, rn: v0..v31, tb: mandated_tb, shift: 0..(esize-1), u_bit: 0..1 }
  relation:
    op: throws
    expr: encode_neon_shll(ops, u_bit, not tb_is_high(tb))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, items: [8b, 16b, 4h, 8h, 2s, 4s] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: "ARM ARM SSHLL Q=0 Tb=8B/4H/2S vs SSHLL2 Q=1 Tb=16B/8H/4S; llvm-mc rejects sshll2 v0.8h, v0.8b, #0"
```

## encode_neon_shll_neg_arity_kinds
- Tier: 4
- Rationale: Body documents `sshll/ushll requires 3 operands` and `unsupported source`. Coverage sweep of the remaining Err arms (arity < 3, Tb not in {8b,16b,4h,8h,2s,4s}, non-reg/non-imm kinds, invalid names). Negative-error with llvm-mc-aligned rejection.
- Seed: neon.rs:1473 `if operands.len() < 3`; neon.rs:1479 unsupported source Err
- Formal: ∀ n < 3. encode_neon_shll(ops[..n], u, h) is Err. ∀ Tb ∉ {8b,16b,4h,8h,2s,4s}. encode_neon_shll([Vd.8h, Vn.Tb, Imm(0)], u, h) is Err. ∀ non-matching kind at a slot. encode_neon_shll is Err. ∀ invalid NEON name at Rd/Rn. encode_neon_shll is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shll
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, ta, slot, which, u_bit, is_high, bad]
  domain: { n: 0..2, rd: v0..v31, rn: v0..v31, ta: unsupported_tb, slot: 0..2, which: 0..5, u_bit: 0..1, is_high: bool, bad: invalid_reg }
  relation:
    op: throws
    expr: encode_neon_shll(ops_too_few_or_bad_kind_or_bad_name, u_bit, is_high)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: "neon.rs:1473 sshll/ushll requires 3 operands; neon.rs:1479 unsupported source; get_neon_reg / get_imm error paths"
```
