# Properties: encode_orn

## encode_orn_diff_reg_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler of the same GNU-style ORN text). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree ORN decoder. encode_eon rejected (same-job gate: EON / opc=10). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping operands <-> `orn Rd, Rn, Rm{, shift}`.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0, 31+32·is_64]. encode_orn([Rd, Rn, Rm, Shift(kind,amt)]) = Word(v) ∧ llvm-mc(-triple=aarch64, "orn Rd, Rn, Rm, kind #amt") = v
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, kind: {lsl,lsr,asr,ror}, amt: 0..63 co-gen with is_64 }
  relation:
    op: eq
    lhs: encode_orn(ops)
    rhs: llvm_mc_word("orn Rd, Rn, Rm, kind #amt")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:14 same textual assembly as gas; README.md:214 orn under Data Processing; data_processing.rs:909 Encode ORN; ARM ARM Logical (shifted register) ORN sf 01 01010 shift N=1 Rm imm6 Rn Rd
```

## encode_orn_diff_neon_llvm_mc
- Tier: 2
- Rationale: README.md:224 lists orn under NEON three-same. Differential vs llvm-mc for T in {8b,16b}. encode_logical NEON ORR is a different opcode (size=10 vs 11). SUT-boundary: same GNU-style assembler helper; mapping <-> `orn Vd.T, Vn.T, Vm.T`.
- Seed: (none)
- Formal: ∀ d, n, m ∈ {0..31}, T ∈ {8b,16b}. encode_orn([Vd.T, Vn.T, Vm.T]) = Word(v) ∧ llvm-mc("orn Vd.T, Vn.T, Vm.T") = v
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: differential
predicate:
  quantifier: forall
  vars: [d, n, m, t]
  domain: { d: 0..31, n: 0..31, m: 0..31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_orn(ops)
    rhs: llvm_mc_word("orn Vd.T, Vn.T, Vm.T")
generators:
  d: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: README.md:224 orn under NEON three-same; data_processing.rs:922 ARM ARM Advanced SIMD three-same ORN 0 Q 0 01110 11 1 Rm 000111 Rn Rd
```

## encode_orn_diff_imm_llvm_mc
- Tier: 2
- Rationale: GNU as / llvm-mc accept `orn Rd, Rn, #imm` as the alias `orr Rd, Rn, #~imm` (logical immediate). README.md:14 "accepts the same textual assembly that GCC's gas would consume". Dispatch `"orn" => encode_orn` so this function owns that text.
- Seed: (none)
- Formal: ∀ rd ∈ {0..30}, rn ∈ {0..31}, is_64 ∈ Bool, imm a valid AArch64 inverted-bitmask. encode_orn([Rd, Rn, Imm(imm)]) = Word(v) ∧ llvm-mc("orn Rd, Rn, #imm") = v
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_64=false, seed=0 — orn w0, w0, #0xaaaaaaaa
- Bug report: pbt-out/bug_reports/encode_orn_imm_alias.md

```property
function: encode_orn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, imm]
  domain: { rd: 0..30, rn: 0..31, imm: valid inverted bitmask }
  relation:
    op: eq
    lhs: encode_orn([Rd, Rn, Imm(imm)])
    rhs: llvm_mc_word("orn Rd, Rn, #imm")
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 18446744073709551615, type: u64 }
evidence: README.md:14 gas-compatible GNU-style assembly; llvm-mc orn x0, x1, #1 encodes as orr x0, x1, #0xfffffffffffffffe
```

## encode_orn_metamorphic_n_bit_vs_orr
- Tier: 4c
- Rationale: ARM ARM Logical (shifted register) ORN is ORR with N=1 (bit 21). encode_logical(opc=01) is ORR (N=0), different job so not a differential sibling; the N-bit relation is an independent field metamorphic. Required metamorphic companion to the differential.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt in range. encode_orn(ops) XOR encode_logical(ops, opc=01) = 1<<21
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_orn(ops) XOR encode_logical(ops, 0b01)
    rhs: 1 << 21
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Logical (shifted register) N at bit 21; ORN N=1 vs ORR N=0 with opc=01; data_processing.rs:933 N=1
```

## encode_orn_metamorphic_mvn_alias
- Tier: 4c
- Rationale: Documented alias at data_processing.rs:753: MVN Rd, Rm = ORN Rd, XZR, Rm. encode_mvn is same-job on this subset. Also required to match llvm-mc `orn Rd, ZR, Rm` / `mvn Rd, Rm`.
- Seed: (none)
- Formal: ∀ rd, rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt in range. encode_orn([Rd, ZR, Rm, shift]) = encode_mvn([Rd, Rm, shift]) = llvm-mc("orn Rd, ZR, Rm, shift")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, amt]
  domain: { rd: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_orn([Rd, ZR, Rm, shift])
    rhs: encode_mvn([Rd, Rm, shift])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: data_processing.rs:753 MVN Rd, Rm -> ORN Rd, XZR, Rm; llvm-mc orn x0, xzr, x1 encodes as mvn x0, x1
```

## encode_orn_invariant_arm_fields
- Tier: 4d
- Rationale: ARM ARM field layout of shifted-register ORN is an exact structural predicate on every success-path GPR word. Stronger differential already covers value equality vs llvm-mc; this pins each field independently. Bounds 0/1/31/32/63 sampled exactly via amt generator.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool, kind, amt in range. word sf=is_64 ∧ opc=01 ∧ bits[28:24]=01010 ∧ shift=kind ∧ N=1 ∧ Rm=rm ∧ imm6=amt ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, amt: 0..(31+32*is_64) }
  relation:
    op: holds
    expr: sf==(is_64) && opc==0b01 && op==0b01010 && N==1 && Rm==rm && imm6==amt && Rn==rn && Rd==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Logical (shifted register) ORN sf 01 01010 shift 1 Rm imm6 Rn Rd; data_processing.rs:933
```

## encode_orn_neg_arity_invalid_name
- Tier: 4e
- Rationale: llvm-mc rejects fewer than 3 operands ("too few operands") and invalid register names (x32, foo, empty, r0). Documented arity at data_processing.rs:911 "orn requires 3 operands".
- Seed: (none)
- Formal: ∀ n ∈ {0,1,2}. encode_orn(n GPRs) = Err. ∀ bad ∈ {x32,w32,foo,"",r0,x}. encode_orn([Rd, Rn, Reg(bad)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, bad]
  domain: { n: 0..2, bad: {x32,w32,foo,empty,r0,x} }
  relation:
    op: holds
    expr: encode_orn(ops).is_err()
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  bad: { gen: oneof, items: ["x32", "w32", "foo", "", "r0", "x"] }
expected_error: String
evidence: llvm-mc too few operands; data_processing.rs:911 orn requires 3 operands; parse_reg_num rejects x32/foo
```

## encode_orn_neg_error_contracts
- Tier: 4e
- Rationale: llvm-mc / ARM ARM reject mixed X/W, SP/WSP (register 31 is ZR not SP), FP/SIMD names as GPRs, out-of-range imm6, unknown shift kinds, trailing non-shift 4th operand, and NEON T not in {8B,16B}. Implemented as encode_orn_neg_extra_operand / mixed_width / sp_fp / shift_range / unknown_shift / invalid_neon_arr.
- Seed: (none)
- Formal: ∀ mixed-width / SP / FP / amt out of range / unknown shift / extra non-shift / T not in {8b,16b}. encode_orn(ops) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: extra which=0 (w0,w0,w0,w0); mixed rd64=false rn64=false rm64=true; wsp at operand 0; lsl #32 on W; unknown lslx; T=8h
- Bug report: pbt-out/bug_reports/encode_orn_extra_operand.md; pbt-out/bug_reports/encode_orn_mixed_width.md; pbt-out/bug_reports/encode_orn_sp_as_zr.md; pbt-out/bug_reports/encode_orn_fp_as_gpr.md; pbt-out/bug_reports/encode_orn_shift_out_of_range.md; pbt-out/bug_reports/encode_orn_unknown_shift_kind.md; pbt-out/bug_reports/encode_orn_invalid_neon_arr.md

```property
function: encode_orn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30 }
  relation:
    op: holds
    expr: encode_orn(invalid_ops).is_err()
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects mixed width, SP, FP, lsl #32 on W, lsl #64 on X, extra operand, T=4h/8h/2s/4s/2d; ARM ARM register 31 is XZR/WZR; ARM ARM T in 8B/16B
```

## encode_orn_metamorphic_sf
- Tier: 4c
- Rationale: Coverage sweep. ARM ARM sf is bit 31 of Logical (shifted register). Equal-number X vs W encodings must differ only in sf. amt in 0..31 so both widths are allocated.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..30}, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0,31]. encode_orn(X-ops) XOR encode_orn(W-ops) = 1<<31
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, amt]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, amt: 0..31 }
  relation:
    op: eq
    lhs: encode_orn(ops_x) XOR encode_orn(ops_w)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Logical (shifted register) sf at bit 31
```

## encode_orn_metamorphic_neon_q
- Tier: 4c
- Rationale: Coverage sweep. ARM ARM Q is bit 30 of Advanced SIMD three-same ORN; T=8B (Q=0) vs T=16B (Q=1) at equal Rd/Rn/Rm must differ only in Q.
- Seed: (none)
- Formal: ∀ d, n, m ∈ {0..31}. encode_orn(T=8b) XOR encode_orn(T=16b) = 1<<30
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [d, n, m]
  domain: { d: 0..31, n: 0..31, m: 0..31 }
  relation:
    op: eq
    lhs: encode_orn(ops_8b) XOR encode_orn(ops_16b)
    rhs: 1 << 30
generators:
  d: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Advanced SIMD three-same Q at bit 30; data_processing.rs:921 q = 1 iff arr_d == 16b
```

## encode_orn_neg_neon_mismatch_or_bare
- Tier: 4e
- Rationale: Coverage sweep. llvm-mc rejects mismatched Vd/Vn/Vm arrangements and bare GPR/V names in vector ORN. ARM ARM requires T in {8B,16B} matching across operands.
- Seed: (none)
- Formal: ∀ d, n, m ∈ {0..31}. encode_orn([Vd.8b, Vn.16b, Vm.8b]) = Err ∧ encode_orn([Vd.8b, Reg(Vn), Vm.8b]) = Err ∧ encode_orn([Vd.8b, Vn.8b, Reg(Xm)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: d=0, n=0, m=0, which=0 — orn v0.8b, v0.16b, v0.8b
- Bug report: pbt-out/bug_reports/encode_orn_neon_mismatch.md

```property
function: encode_orn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [d, n, m, which]
  domain: { d: 0..31, n: 0..31, m: 0..31, which: 0..2 }
  relation:
    op: holds
    expr: encode_orn(ops).is_err()
generators:
  d: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: llvm-mc orn v0.8b, v1.16b, v2.8b is invalid operand; ARM ARM T must match
```

## encode_orn_neg_trailing_after_shift
- Tier: 4e
- Rationale: Coverage sweep. llvm-mc rejects a 5th operand after a valid shift. Extra operands after the optional shift are not part of the ORN grammar.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..30}, is_64 ∈ Bool, extra ∈ {Reg, Imm, Symbol}. encode_orn([Rd, Rn, Rm, LSL #1, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=rn=rm=0, is_64=false, extra=Reg(w0) — orn w0, w0, w0, lsl #1, w0
- Bug report: pbt-out/bug_reports/encode_orn_trailing_after_shift.md

```property
function: encode_orn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30 }
  relation:
    op: holds
    expr: encode_orn([Rd, Rn, Rm, Shift(lsl,1), extra]).is_err()
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects trailing tokens after a valid shift
```

## encode_orn_neg_non_reg_kinds
- Tier: 4e
- Rationale: Coverage sweep. get_reg documents "expected register" for non-Reg kinds. Imm/Mem/Symbol/Cond at a GPR slot of shifted-register ORN must Err (llvm-mc rejects them).
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, kind ∈ {Imm, Mem, Symbol, Cond}. encode_orn(ops with that slot replaced) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_orn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..2, kind: {Imm, Mem, Symbol, Cond} }
  relation:
    op: holds
    expr: encode_orn(ops).is_err()
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand; get_reg expected register at operand idx
```
