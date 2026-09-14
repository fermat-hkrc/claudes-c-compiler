# Property ledger: encode_rev32

## encode_rev32_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree REV32 decoder). Sibling encode_neon_two_misc rejected for scalar (vector class; dispatch splits). Sibling encode_rev/encode_rev16/encode_rbit/encode_clz/encode_cls rejected (different opcode). Doc evidence: README.md:11 GNU-style assembly; README.md:240 lists rev32; encoder/mod.rs:579-581 dispatch; ARM ARM Data-processing (1 source) REV32 Xd,Xn only.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_diff_valid_gpr
- Formal: ∀ rd,rn ∈ {0..31}. encode_rev32([Reg(x{rd}|xzr), Reg(x{rn}|xzr)]) = llvm-mc("rev32 Xd, Xn") as LE word
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: eq
    lhs: encode_rev32([Reg(gpr64(rd)), Reg(gpr64(rn))])
    rhs: llvm_mc_word("rev32 " + gpr64(rd) + ", " + gpr64(rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:11 GNU-style gas contract; ARM ARM REV32 Xd,Xn; llvm-mc -triple=aarch64 -show-encoding
```

## encode_rev32_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM encoding diagram. Stronger rejected as above. Weaker than differential but pins each field independently (sf=1, opcode=000010).
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}. let w = encode_rev32([Reg(x{rd}), Reg(x{rn})]). w = (1<<31)|(1<<30)|(0b011010110<<21)|(0b000010<<10)|(rn<<5)|rd ∧ w[31]=1 ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=11010110 ∧ w[20:16]=00000 ∧ w[15:10]=000010 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: eq
    lhs: encode_rev32([Reg(gpr64(rd)), Reg(gpr64(rn))])
    rhs: 0xDAC00800 | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (1 source) REV32 encoding 1 1 0 11010110 00000 000010 Rn Rd
```

## encode_rev32_metamorphic_rd_rn
- Tier: 4
- Rationale: Algebraic metamorphic: incrementing Rd/Rn must touch only that 5-bit field. Stronger rejected as above. No W-vs-X sf metamorphic — ARM forbids the 32-bit form.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_metamorphic_rd_rn_sf
- Formal: ∀ rd,rn ∈ {0..30}. let b = encode_rev32(x{rd}, x{rn}). encode_rev32(x{rd+1}, x{rn}) & 0x1f = rd+1 ∧ that word agrees with b outside bits[4:0]; encode_rev32(x{rd}, x{rn+1}) bits[9:5] = rn+1 ∧ agrees with b outside bits[9:5]
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: u32 0..=30, rn: u32 0..=30 }
  relation:
    op: holds
    expr: field_independence(encode_rev32, rd, rn)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM REV32 field layout Rd bits[4:0] Rn bits[9:5]
```

## encode_rev32_diff_neon
- Tier: 2
- Rationale: encode_rev32 contains a documented NEON path (purpose comment bitfield.rs:206). Differential vs llvm-mc for T in {8B,16B,4H,8H} (ARM ARM Advanced SIMD two-register miscellaneous REV32). encode_neon_two_misc is the public dispatch sibling, not an independent reference (same-job gate: shared helpers; llvm-mc is independent).
- Seed: bitfield.rs encode_rbit_pbt NEON differential
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h}. encode_rev32([RegArrangement(v{rd}.T), RegArrangement(v{rn}.T)]) = llvm-mc("rev32 v{rd}.T, v{rn}.T")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, arr]
  domain: { rd: u32 0..=31, rn: u32 0..=31, arr: {8b,16b,4h,8h} }
  relation:
    op: eq
    lhs: encode_rev32([RegArrangement(v{rd}, arr), RegArrangement(v{rn}, arr)])
    rhs: llvm_mc_word("rev32 v{rd}." + arr + ", v{rn}." + arr)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, items: ["8b", "16b", "4h", "8h"] }
evidence: ARM ARM Advanced SIMD two-register miscellaneous REV32 T in {8B,16B,4H,8H}; README.md:225 lists rev32 under NEON two-misc
```

## encode_rev32_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects fewer than 2 operands. get_reg fails when the slot is missing.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_arity
- Formal: ∀ len ∈ {0,1}, rd,rn ∈ {0..31}. encode_rev32(ops[:len]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, rd, rn]
  domain: { len: usize 0..=1, rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: throws
    expr: encode_rev32(truncate([Reg(x{rd}), Reg(x{rn})], len))
expected_error: String
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects too few operands for rev32; ARM ARM REV32 requires Xd, Xn
```

## encode_rev32_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 3rd operand. README.md:11 gas-compatible assembly.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_extra_operand
- Formal: ∀ rd,rn ∈ {0..31}, extra ∈ Operand. encode_rev32([Reg(x{rd}), Reg(x{rn}), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=Reg("x0") — encode_rev32 returns Ok(Word) instead of Err (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_rev32_extra_operand.md

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: u32 0..=31, rn: u32 0..=31, extra: Operand }
  relation:
    op: throws
    expr: encode_rev32([Reg(x{rd}), Reg(x{rn}), extra])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: [Reg, Imm, Shift, RegArrangement] }
evidence: llvm-mc "invalid operand" for rev32 x0, x1, x2; README.md:11 gas contract
```

## encode_rev32_neg_w32
- Tier: 4
- Rationale: Negative/error contract. ARM ARM REV32 is 64-bit only (no W form). llvm-mc rejects `rev32 w0, w1`. Purpose comment bitfield.rs:218 "REV32 is 64-bit only". Bound 31 sampled exactly (wzr).
- Seed: (none) — encode_rev16 allows W; this is the distinguishing REV32 contract
- Formal: ∀ rd,rn ∈ {0..31}. encode_rev32([Reg(w{rd}|wzr), Reg(w{rn}|wzr)]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0 — encode_rev32([w0, w0]) returns Ok(0xdac00800) instead of Err (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_rev32_w32.md

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: throws
    expr: encode_rev32([Reg(gpr32(rd)), Reg(gpr32(rn))])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM REV32 <Xd>, <Xn> only; llvm-mc rejects rev32 w0, w1; bitfield.rs:218 purpose comment "REV32 is 64-bit only"
```

## encode_rev32_neg_sp
- Tier: 4
- Rationale: Negative/error contract. ARM ARM register 31 is ZR not SP. llvm-mc rejects SP/WSP.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other ∈ {0..30}. encode_rev32(ops with slot which = SP) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp=wsp, other=0 — encode_rev32([wsp, x0]) returns Ok instead of Err (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_rev32_sp.md

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, other]
  domain: { which: u32 0..=1, sp64: bool, other: u32 0..=30 }
  relation:
    op: throws
    expr: encode_rev32(ops_with_sp(which, sp64, other))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM register 31 is ZR not SP; llvm-mc rejects rev32 sp, x0
```

## encode_rev32_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — documented GNU aliases x31/XZR/LR/uppercase must agree with llvm-mc. Differential. Bound 31 (xzr/x31) sampled exactly.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, dest_spell,src_spell ∈ {0..4}. encode_rev32([Reg(spell(rd)), Reg(spell(rn))]) = llvm-mc("rev32 " + spell(rd) + ", " + spell(rn))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_spell, src_spell]
  domain: { rd: u32 0..=31, rn: u32 0..=31, dest_spell: u32 0..=4, src_spell: u32 0..=4 }
  relation:
    op: eq
    lhs: encode_rev32([Reg(spell(rd, dest_spell)), Reg(spell(rn, src_spell))])
    rhs: llvm_mc_word("rev32 " + spell(rd, dest_spell) + ", " + spell(rn, src_spell))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style assembly; parse_reg_num maps lr/x31/xzr; llvm-mc accepts those aliases
```

## encode_rev32_neg_mixed_width
- Tier: 4
- Rationale: Sweep — llvm-mc rejects mixed W/X. ARM ARM REV32 requires both operands Xd,Xn.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_mixed_width
- Formal: ∀ rd,rn ∈ {0..31}, rd64 ≠ rn64. encode_rev32([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — encode_rev32([x0, w0]) returns Ok (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_rev32_mixed_width.md

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: u32 0..=31, rn: u32 0..=31, rd64: bool, rn64: bool }
  relation:
    op: throws
    expr: encode_rev32([Reg(gpr(rd64, rd)), Reg(gpr(rn64, rn))]) when rd64 != rn64
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
evidence: llvm-mc rejects rev32 x0, w1; ARM ARM REV32 Xd,Xn only
```

## encode_rev32_neg_fp
- Tier: 4
- Rationale: Sweep — FP/SIMD prefixes are not GPR operands for scalar REV32. llvm-mc rejects d/s/q/v/h/b.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_rev32(ops with slot which = prefix{n}) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — encode_rev32([d0, x1]) returns Ok(0xdac00820) (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_rev32_fp.md

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: u32 0..=1, prefix: {d,s,q,v,h,b}, n: u32 0..=31 }
  relation:
    op: throws
    expr: encode_rev32(ops_with_fp(which, prefix, n))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, items: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects rev32 d0, d1; ARM ARM REV32 takes GPRs
```

## encode_rev32_neg_nonreg
- Tier: 4
- Rationale: Sweep — wrong operand kind must Err. get_reg rejects non-Reg.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Shift, Mem, Label, Symbol, Cond}. encode_rev32(ops with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: u32 0..=1, bad: non-Reg Operand }
  relation:
    op: throws
    expr: encode_rev32(ops_with_kind(which, bad))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, items: [Imm, Shift, Mem, Label, Symbol, Cond] }
evidence: get_reg requires Operand::Reg; llvm-mc rejects non-register operands
```

## encode_rev32_neg_invalid_name
- Tier: 4
- Rationale: Sweep — invalid register names must Err. Bound x32 (just past 31) sampled.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_rev32(ops with slot which = name) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: u32 0..=1, name: invalid register names }
  relation:
    op: throws
    expr: encode_rev32(ops_with_name(which, name))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, items: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
evidence: parse_reg_num returns None for these; llvm-mc rejects them
```

## encode_rev32_neg_neon_invalid_arr
- Tier: 4
- Rationale: Sweep — ARM ARM REV32 vector T is only {8B,16B,4H,8H}. llvm-mc rejects 2s/4s/2d/1d. neon_arr_to_q_size accepts those sizes.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {2s,4s,2d,1d}. encode_rev32([RegArrangement(v{rd}.T), RegArrangement(v{rn}.T)]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, arr="2s" — encode_rev32 returns Ok instead of Err (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_rev32_neon_invalid_arr.md

```property
function: encode_rev32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, arr]
  domain: { rd: u32 0..=31, rn: u32 0..=31, arr: {2s,4s,2d,1d} }
  relation:
    op: throws
    expr: encode_rev32([RegArrangement(v{rd}, arr), RegArrangement(v{rn}, arr)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, items: ["2s", "4s", "2d", "1d"] }
evidence: ARM ARM Advanced SIMD REV32 T in {8B,16B,4H,8H}; llvm-mc rejects rev32 v0.4s, v1.4s
```
