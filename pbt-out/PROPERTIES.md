# Properties: encode_neon_three_diff_narrow

## encode_neon_three_diff_narrow_diff_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). In-tree ADDHN decoder does not exist so algebraic round-trip is unavailable. encode_neon_three_diff is a widening/long sibling with a different source-arrangement map and fails the same-job sibling gate. Doc evidence: assembler README "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:628-635 addhn family dispatch; ARM ARM Advanced SIMD three-different encoding.
- Seed: (none) — no existing tests for encode_neon_three_diff_narrow
- Formal: ∀ rd,rn,rm ∈ 0..=31, ∀ (Ta,Tb,is_high) ∈ {(8h,8b,false),(8h,16b,true),(4s,4h,false),(4s,8h,true),(2d,2s,false),(2d,4s,true)}, ∀ (mnemonic,U,opcode) ∈ {(addhn,0,0b0100),(raddhn,1,0b0100),(subhn,0,0b0110),(rsubhn,1,0b0110)}. encode_neon_three_diff_narrow([Vd.Tb,Vn.Ta,Vm.Ta], U, opcode, is_high) = Word(llvm-mc("{mnemonic}{2?} Vd.Tb, Vn.Ta, Vm.Ta")).
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, tb, is_high, u_bit, opcode]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, arr: valid_addhn_ta_tb, mnemonic: addhn_family }
  relation:
    op: eq
    lhs: encode_neon_three_diff_narrow([RegArrangement(vd,tb), RegArrangement(vn,ta), RegArrangement(vm,ta)], u_bit, opcode, is_high) as Word
    rhs: llvm_mc("{mnem} vd.tb, vn.ta, vm.ta")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:628-635 addhn dispatch; neon.rs:1513 ARM ARM three-different format 0 Q U 01110 size 1 Rm opcode 00 Rn Rd
```

## encode_neon_three_diff_narrow_q_bit_is_high
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM Q is bit 30 and is 1 iff the `*2` (upper-half) variant. Stronger differential already covers the happy path; this isolates the is_high→Q contract independently of llvm-mc. State machine / round-trip rejected as above.
- Seed: (none)
- Formal: ∀ valid 3-reg ADDHN operands, ∀ u ∈ {0,1}, ∀ opcode ∈ {0b0100,0b0110}. encode(..., is_high=true) XOR encode(..., is_high=false) = 1<<30, and both succeed.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, u_bit, opcode]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, ta: {8h,4s,2d}, u_bit: {0,1}, opcode: {0b0100,0b0110} }
  relation:
    op: eq
    lhs: encode_neon_three_diff_narrow(ops, u_bit, opcode, true) XOR encode_neon_three_diff_narrow(ops, u_bit, opcode, false)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
evidence: neon.rs:1513 Format 0 Q U 01110; encoder/mod.rs:628-635 is_high true for *2 mnemonics; ARM ARM Q=1 writes upper half
```

## encode_neon_three_diff_narrow_u_bit
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM U is bit 29 (0=ADDHN/SUBHN, 1=RADDHN/RSUBHN). Isolates the U field independently of llvm-mc.
- Seed: (none)
- Formal: ∀ valid 3-reg ADDHN operands, ∀ is_high ∈ Bool, ∀ opcode ∈ {0b0100,0b0110}. encode(..., u=1) XOR encode(..., u=0) = 1<<29, and both succeed.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, is_high, opcode]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, ta: {8h,4s,2d}, is_high: Bool, opcode: {0b0100,0b0110} }
  relation:
    op: eq
    lhs: encode_neon_three_diff_narrow(ops, 1, opcode, is_high) XOR encode_neon_three_diff_narrow(ops, 0, opcode, is_high)
    rhs: 1 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  is_high: { gen: bool }
  opcode: { gen: oneof, options: [4, 6] }
evidence: neon.rs:1513 U at bit 29; encoder/mod.rs:628-635 U=1 for raddhn/rsubhn; ARM ARM U field
```

## encode_neon_three_diff_narrow_word_layout
- Tier: 4
- Rationale: Algebraic invariant from the documented bit layout: bit31=0, bits[28:24]=01110, bit21=1, bits[11:10]=00, Rd/Rn/Rm/size/opcode placed as specified. Stronger differential already covers numeric equality; this pins each field so a swapped Rn/Rm would fail even if llvm-mc were unavailable.
- Seed: (none)
- Formal: ∀ valid 3-reg ADDHN operands, ∀ u ∈ {0,1}, ∀ opcode ∈ {0b0100,0b0110}, ∀ is_high ∈ Bool. let w = encode(...). w[31]=0 ∧ w[30]=is_high ∧ w[29]=u ∧ w[28:24]=0b01110 ∧ w[23:22]=size(Ta) ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:12]=opcode ∧ w[11:10]=0 ∧ w[9:5]=rn ∧ w[4:0]=rd. size(8h)=00, size(4s)=01, size(2d)=10.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, u_bit, opcode, is_high]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, ta: {8h,4s,2d}, u_bit: {0,1}, opcode: {0b0100,0b0110}, is_high: Bool }
  relation:
    op: holds
    expr: word_fields_match_arm_three_diff_narrow(w, rd, rn, rm, size(ta), u_bit, opcode, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
  is_high: { gen: bool }
evidence: neon.rs:1513 Format 0 Q U 01110 size 1 Rm opcode 00 Rn Rd; ARM ARM Advanced SIMD three different
```

## encode_neon_three_diff_narrow_arity_err
- Tier: 4
- Rationale: Negative/error contract: function returns Err when operands.len() < 3. Evidence: neon.rs:1515 `if operands.len() < 3 { return Err("addhn/subhn requires 3 operands") }`. Documented bound is exactly 3; sample empty, 1, and 2 (bound-1).
- Seed: (none)
- Formal: ∀ ops with len(ops) ∈ {0,1,2}, ∀ u ∈ {0,1}, ∀ opcode ∈ {0b0100,0b0110}, ∀ is_high ∈ Bool. encode_neon_three_diff_narrow(ops, u, opcode, is_high) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, u_bit, opcode, is_high]
  domain: { ops: neon_reg_list_len_0_to_2, u_bit: {0,1}, opcode: {0b0100,0b0110}, is_high: Bool }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow(ops, u_bit, opcode, is_high)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
  is_high: { gen: bool }
evidence: neon.rs:1515 addhn/subhn requires 3 operands
```

## encode_neon_three_diff_narrow_unsupported_src
- Tier: 4
- Rationale: Negative/error contract: source arrangement (operand 1) must be 8h/4s/2d; any other Ta is Err. Evidence: neon.rs:1519-1520. ARM ARM size=11 reserved; Ta of 8B/16B/4H/2S/1D are not ADDHN sources. Bound: valid set {8h,4s,2d}; generate outside that closed set.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ 0..=31, ∀ Ta ∉ {8h,4s,2d} ∪ {empty}, ∀ u,opcode,is_high in the dispatch domain. encode([Vd.8b, Vn.Ta, Vm.Ta], ...) is Err containing "unsupported source".
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, u_bit, opcode, is_high]
  domain: { ta: invalid_addhn_source_arrangement }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow([Vd.8b, Vn.ta, Vm.ta], u_bit, opcode, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8b, 16b, 4h, 8h_invalid_as_only_when_paired_wrong, 2s, 1d, 2d_ok_excluded, empty, 4d, 8s] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
  is_high: { gen: bool }
evidence: neon.rs:1519-1520 addhn: unsupported source; ARM ARM ADDHN Ta in {8H,4S,2D}
```

## encode_neon_three_diff_narrow_dest_tb_must_match
- Tier: 5
- Rationale: Negative/error (and differential rejection) contract from ARM ARM / gas: dest Tb is determined by Ta and Q. ADDHN Vd.Tb, Vn.Ta, Vm.Ta with wrong Tb is rejected by llvm-mc/gas. README claims gas-compatible assembly. SUT currently ignores dest arrangement — this property asserts Err (or agreement with llvm-mc reject) when Tb is not the mandated pairing. Stronger full differential on invalid text is the same job.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ 0..=31, ∀ (Ta,is_high) valid, ∀ Tb such that Tb ≠ mandated_tb(Ta,is_high) and Tb is a NEON arrangement. llvm-mc rejects the asm ∧ encode_neon_three_diff_narrow([Vd.Tb,Vn.Ta,Vm.Ta], ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: addhn2 v0.8b, v0.8h, v0.8h (rd=rn=rm=0, Ta=8h, Tb=8b, is_high=true, U=0, opcode=0b0100)
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_narrow_mismatched_dest_tb.md

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, tb, is_high, u_bit, opcode]
  domain: { tb: mismatched_addhn_dest_arrangement, ta: valid_addhn_source }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow([Vd.tb, Vn.ta, Vm.ta], u_bit, opcode, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  tb: { gen: oneof, options: [8b, 16b, 4h, 8h, 2s, 4s, 1d, 2d] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
evidence: ARM ARM ADDHN Vd.Tb,Vn.Ta,Vm.Ta pairing; README.md:5-14 gas-compatible; llvm-mc rejects mismatched Tb
```

## encode_neon_three_diff_narrow_non_reg_err
- Tier: 4
- Rationale: Negative/error contract: each of the three slots must be a NEON register (RegArrangement or Reg). Imm/Mem/Symbol/Shift/Cond/etc. at any slot is Err via get_neon_reg. Evidence: neon.rs:7-20 get_neon_reg `expected NEON register at operand N`.
- Seed: (none)
- Formal: ∀ slot ∈ {0,1,2}, ∀ non-reg Operand o, ∀ valid fillers in the other two slots. encode(ops with ops[slot]=o, ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, bad, rd, rn, rm, u_bit, opcode, is_high]
  domain: { slot: {0,1,2}, bad: Imm|Mem|Symbol|Shift|Cond|Label|Barrier }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow(ops_with_bad_at_slot, u_bit, opcode, is_high)
expected_error: String
generators:
  slot: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
  is_high: { gen: bool }
evidence: neon.rs:7-20 get_neon_reg expected NEON register; ARM ARM three register form
```

## encode_neon_three_diff_narrow_rm_ta_must_match
- Tier: 5
- Rationale: Negative/error contract from ARM ARM / gas: Vm.Ta must equal Vn.Ta. llvm-mc rejects mixed source arrangements. README claims gas-compatible assembly. SUT discards Rm arrangement.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ 0..=31, ∀ Ta_n ≠ Ta_m ∈ {8h,4s,2d}, ∀ is_high, u, opcode in dispatch domain. llvm-mc rejects the asm ∧ encode([Vd.Tb(Ta_n), Vn.Ta_n, Vm.Ta_m], ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: addhn v0.4h, v0.4s, v0.8h (rd=rn=rm=0, Ta_n=4s, Ta_m=8h, is_high=false, U=0, opcode=0b0100)
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_narrow_rm_ta_mismatch.md

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta_n, ta_m, is_high, u_bit, opcode]
  domain: { ta_n: valid_addhn_source, ta_m: valid_addhn_source_neq_ta_n }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow([Vd.tb, Vn.ta_n, Vm.ta_m], u_bit, opcode, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta_n: { gen: oneof, options: [8h, 4s, 2d] }
  ta_m: { gen: oneof, options: [8h, 4s, 2d] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
evidence: ARM ARM ADDHN Vd.Tb,Vn.Ta,Vm.Ta same Ta; README.md:5-14 gas-compatible; llvm-mc rejects mismatched Rm Ta
```

## encode_neon_three_diff_narrow_extra_operand_err
- Tier: 5
- Rationale: Negative/error contract: ARM ADDHN is a 3-register instruction. llvm-mc rejects a fourth operand. SUT only checks len < 3.
- Seed: (none)
- Formal: ∀ valid 3-reg ADDHN ops, ∀ extra NEON reg. encode(ops ++ [extra], ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: addhn v0.8b, v0.8h, v0.8h, v0.8h (four RegArrangement operands, U=0, opcode=0b0100, is_high=false)
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_narrow_extra_operand.md

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, ta, is_high, u_bit, opcode]
  domain: { extra: neon_vreg }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow([Vd.tb, Vn.ta, Vm.ta, Vextra.ta], u_bit, opcode, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
evidence: ARM ARM three-register ADDHN; README.md:5-14 gas-compatible; llvm-mc rejects 4th operand
```

## encode_neon_three_diff_narrow_gpr_dest_err
- Tier: 5
- Rationale: Negative/error contract: dest must be Vd.Tb. GPR/FP names (x/w/d/s/q/h/b) are rejected by llvm-mc. get_neon_reg accepts Operand::Reg via parse_reg_num.
- Seed: (none)
- Formal: ∀ prefix ∈ {x,w,d,s,q,h,b}, ∀ n ∈ 0..=31, ∀ valid Rn/Rm Ta. encode([Reg(prefix n), Vn.Ta, Vm.Ta], ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: addhn x0, v0.8h, v0.8h (prefix=x, n=0, Ta=8h, is_high=false, U=0, opcode=0b0100)
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_narrow_gpr_dest.md

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prefix, n, rn, rm, ta, is_high, u_bit, opcode]
  domain: { prefix: gpr_or_fp_prefix, n: u32_0_31 }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow([Reg(prefix n), Vn.ta, Vm.ta], u_bit, opcode, is_high)
expected_error: String
generators:
  prefix: { gen: oneof, options: [x, w, d, s, q, h, b] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: [8h, 4s, 2d] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
evidence: ARM ARM Vd.Tb destination; README.md:5-14 gas-compatible; llvm-mc rejects addhn x0, ...
```

## encode_neon_three_diff_narrow_invalid_reg_err
- Tier: 4
- Rationale: Negative/error contract: get_neon_reg returns Err when parse_reg_num fails (v32, empty, non-register names). Coverage-sweep of the invalid-register branch of get_neon_reg used by all three slots.
- Seed: (none)
- Formal: ∀ slot ∈ {0,1,2}, ∀ bad ∈ {v32,v99,foo,"",v,v-1}, ∀ u,opcode,is_high. encode(ops with RegArrangement(bad, ...) at slot) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, bad, u_bit, opcode, is_high, rd]
  domain: { slot: {0,1,2}, bad: invalid_neon_reg_name }
  relation:
    op: throws
    expr: encode_neon_three_diff_narrow(ops_with_bad_reg_at_slot, u_bit, opcode, is_high)
expected_error: String
generators:
  slot: { gen: int, min: 0, max: 2, type: usize }
  bad: { gen: oneof, options: [v32, v99, foo, empty, v, v-1] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, options: [4, 6] }
  is_high: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:7-12 get_neon_reg invalid NEON register; parse_reg_num rejects n>31
```
