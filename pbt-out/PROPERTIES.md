# Properties: encode_neon_fcvtn

## encode_neon_fcvtn_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FCVTN decoder). Sibling encode_neon_fcvtl rejected (same-job gate: widening vs narrowing). encode_neon_xtl / encode_neon_two_misc rejected (different encoding class). README.md:12 claims gas-compatible textual assembly; encoder/mod.rs:3 claims 32-bit AArch64 words. llvm-mc is the independent reference for that contract.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (tb,ta,is_high) ∈ {(4h,4s,false),(8h,4s,true),(2s,2d,false),(4s,2d,true)}. encode_neon_fcvtn([Vd.tb, Vn.ta], is_high) = llvm-mc("fcvtn{2} Vd.tb, Vn.ta")
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, is_high]
  domain: { rd: v0..v31, rn: v0..v31, (tb,ta,is_high): ARM FCVTN pairs }
  relation:
    op: eq
    lhs: encode_neon_fcvtn([arr(rd,tb), arr(rn,ta)], is_high)
    rhs: llvm_mc(asm2(rd, rn, tb, ta, is_high))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: string }
  ta: { gen: string }
  is_high: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtn_meta_rd_rn_q
- Tier: 4
- Rationale: Metamorphic isolation of Rd/Rn/Q. Stronger differential is the primary property; this checks field packing independently of llvm-mc. Changing only Rd (resp. Rn, is_high) must XOR only bits[4:0] (resp. bits[9:5], bit 30).
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_meta_rd_rn_q
- Formal: ∀ rd,rd2,rn,rn2 ∈ {0..31}, ∀ valid (tb,ta,is_high). let w = encode_neon_fcvtn([Vd.tb,Vn.ta], is_high). w ⊕ encode([Vd2.tb,Vn.ta], is_high) = rd ⊕ rd2 ∧ w ⊕ encode([Vd.tb,Vn2.ta], is_high) = (rn ⊕ rn2)<<5 ∧ w ⊕ encode([Vd.tb',Vn.ta], !is_high) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rd2, rn, rn2, tb, ta, is_high]
  domain: { rd,rd2,rn,rn2: v0..v31, (tb,ta,is_high): ARM FCVTN pairs }
  relation:
    op: eq
    lhs: sut_word(ops2(rd, rn, tb, ta), is_high) ^ sut_word(ops2(rd2, rn, tb, ta), is_high)
    rhs: rd ^ rd2
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: string }
  ta: { gen: string }
  is_high: { gen: bool }
evidence: neon.rs:1651
```

## encode_neon_fcvtn_inv_arm_layout
- Tier: 4
- Rationale: ARM two-misc layout invariant. FCVTN is Advanced SIMD two-register miscellaneous with U=0 opcode=10110. Weaker than differential; pins bit fields even if llvm-mc is unavailable. Format sibling neon.rs:1639 for FCVTL uses opcode 10111; FCVTN is the same class with opcode 10110.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_inv_arm_layout
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (tb,ta,is_high). let w = encode_neon_fcvtn(...). bit31(w)=0 ∧ Q(w)=is_high ∧ U(w)=0 ∧ bits[28:24]=01110 ∧ bit23=0 ∧ sz(w)=(ta==2d) ∧ bits[21:17]=10000 ∧ bits[16:12]=10110 ∧ bits[11:10]=10 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, is_high]
  domain: { rd,rn: v0..v31, (tb,ta,is_high): ARM FCVTN pairs }
  relation:
    op: holds
    expr: arm_fcvtn_layout(w, rd, rn, ta, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: string }
  ta: { gen: string }
  is_high: { gen: bool }
evidence: neon.rs:1651
```

## encode_neon_fcvtn_neg_arity
- Tier: 4
- Rationale: FCVTN is a two-operand instruction (Vd.Tb, Vn.Ta). llvm-mc and gas reject 0- and 1-operand forms. Negative-error contract from the GNU-style assembler claim (README.md:12) plus ARM FCVTN syntax.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_neg_arity
- Formal: ∀ rd ∈ {0..31}, ∀ arr ∈ NEON arrangements, ∀ is_high ∈ {false,true}, ∀ n ∈ {0,1}. encode_neon_fcvtn(ops[0..n], is_high) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, arr, is_high, n]
  domain: { rd: v0..v31, arr: neon arrangements, n: 0..1 }
  relation:
    op: throws
    expr: encode_neon_fcvtn(&ops[..n], is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: string }
  n: { gen: int, min: 0, max: 1, type: usize }
  is_high: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtn_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a third operand on FCVTN. README.md:12 claims gas-compatible assembly, so the encoder must Err rather than silently drop the extra operand. No documented arity maximum in the function body; the contract is the public assembler syntax.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, ∀ valid (tb,ta,is_high). llvm-mc("fcvtn{2} Vd.tb, Vn.ta, Ve.tb") = Err ∧ encode_neon_fcvtn([Vd.tb,Vn.ta,Ve.tb], is_high) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: failing
- Counterexample: encode_neon_fcvtn([v0.4h, v0.4s, v0.4h], is_high=false) → Ok(Word(0x0e216800))
- Bug report: pbt-out/bug_reports/encode_neon_fcvtn_extra_operand.md

```property
function: encoder.encode_neon_fcvtn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, tb, ta, is_high]
  domain: { rd,rn,extra: v0..v31, (tb,ta,is_high): ARM FCVTN pairs }
  relation:
    op: throws
    expr: encode_neon_fcvtn(&[arr(rd,tb), arr(rn,ta), arr(extra,tb)], is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: string }
  ta: { gen: string }
  is_high: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtn_neg_mismatched_ta_tb
- Tier: 4
- Rationale: ARM FCVTN admits only four (Tb, Ta, Q) triples. llvm-mc rejects every other arrangement pair. neon.rs:1656 documents a source-arrangement restriction ("fcvtn: unsupported source:") but does not declare dest mismatch invalid in a comment; dest is discarded in the body. The public assembler contract (README.md:12 / ARM FCVTN syntax) requires rejection. Source "2s" is accepted by the coded match but is not an ARM FCVTN source — keep it in the invalid domain.
- Doc contract: neon.rs:1656 "fcvtn: unsupported source:" — domain-restriction fingerprint 3702b777
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_neg_mismatched_ta_tb
- Formal: ∀ rd,rn ∈ {0..31}, ∀ tb,ta ∈ NEON arrangements, ∀ is_high ∈ {false,true}. ¬valid_pair(tb,ta,is_high) ⇒ llvm-mc rejects ∧ encode_neon_fcvtn([Vd.tb,Vn.ta], is_high) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: failing
- Counterexample: encode_neon_fcvtn([v0.8b, v0.2s], is_high=false) → Ok(Word(0x0e216800))
- Bug report: pbt-out/bug_reports/encode_neon_fcvtn_mismatched_ta_tb.md

```property
function: encoder.encode_neon_fcvtn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, is_high]
  domain: { rd,rn: v0..v31, (tb,ta,is_high): NEON arrangements where not ARM FCVTN pair }
  relation:
    op: throws
    expr: encode_neon_fcvtn(&ops2(rd, rn, tb, ta), is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: string }
  ta: { gen: string }
  is_high: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtn_neg_gpr_or_bare
- Tier: 4
- Rationale: FCVTN operands are Vd.Tb / Vn.Ta. llvm-mc rejects GPR prefixes, scalar FP names, and bare v-regs without arrangement. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/h/b; the public assembler contract requires Err.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ {0..31}, ∀ kind ∈ GPR/bare/non-V. llvm-mc rejects the corresponding asm ∧ encode_neon_fcvtn(ops(kind), false) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: failing
- Counterexample: encode_neon_fcvtn([Reg("v0"), v0.4s], is_high=false) → Ok(Word(0x0e216800))  // fcvtn v0, v0.4s
- Bug report: pbt-out/bug_reports/encode_neon_fcvtn_bare_dest.md

```property
function: encoder.encode_neon_fcvtn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, fp_prefix]
  domain: { rd,rn: v0..v31, kind: 0..4, fp_prefix: {x,w,d,s,q,h,b} }
  relation:
    op: throws
    expr: encode_neon_fcvtn(&ops, false)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
  fp_prefix: { gen: string }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtn_diff_alt_spellings
- Tier: 5
- Rationale: Metamorphic/differential: uppercase mnemonic and V/arrangement spellings that llvm-mc accepts must encode identically. README.md:12 gas-compatible textual assembly is case-insensitive for AArch64 mnemonics.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (tb,ta,is_high). encode_neon_fcvtn([V{rd}.tb, V{rn}.ta], is_high) = llvm-mc("FCVTN{2} Vd.TB, Vn.TA")
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, is_high]
  domain: { rd,rn: v0..v31, (tb,ta,is_high): ARM FCVTN pairs }
  relation:
    op: eq
    lhs: encode_neon_fcvtn([Arr(V{rd},tb), Arr(V{rn},ta)], is_high)
    rhs: llvm_mc(uppercase_asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: string }
  ta: { gen: string }
  is_high: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtn_neg_nonreg
- Tier: 4
- Rationale: Sweep of get_neon_reg error path for Imm/Mem/Label. llvm-mc rejects `#0`, `[Xn]`, and labels as FCVTN operands. Documented by get_neon_reg's `expected NEON register` error and README.md:12 gas syntax. Stronger oracles already cover the valid domain.
- Doc contract: neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" — asserted fingerprint cb7b005e
- Seed: encode_neon_fcvtl_pbt.rs:encode_neon_fcvtl_neg_nonreg
- Formal: ∀ rd,rn ∈ {0..31}, ∀ kind ∈ {Imm(0), Mem[Xn], Label}, ∀ slot ∈ {0,1}. encode_neon_fcvtn(ops with slot replaced by kind, false) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd,rn: v0..v31, kind: 0..2, slot: 0..1 }
  relation:
    op: throws
    expr: encode_neon_fcvtn(&ops, false)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```
