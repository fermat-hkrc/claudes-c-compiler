# Properties: encode_ldnp_stnp

## encode_ldnp_stnp_diff_signed_offset_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, which the assembler README claims gas-compatibility with. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree LDNP/STNP decoder). Sibling encode_ldp_stp rejected (same-job gate: LDP/STP pre/post, bits[25:23] in {001,010,011}, shared get_reg).
- Doc contract: load_store.rs:516 "Encoding: opc 101 V 000 L imm7 Rt2 Rn Rt" — asserted fingerprint fdc6a9e2
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_diff_signed_offset_llvm_mc
- Formal: ∀ is_load ∈ Bool, is_64 ∈ Bool, rt1,rt2,rn ∈ 0..31, imm7 ∈ [-64,63]. encode_ldnp_stnp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, imm7·scale}], is_load) = llvm-mc("ldnp/stnp Rt1, Rt2, [Xn|SP, #imm7·scale]") where scale=8 if is_64 else 4, Rt is Xt/Wt (31=XZR/WZR), Rn is Xn|SP (31=SP)
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63 }
  body: encode_ldnp_stnp([Reg(gp(is_64,rt1)), Reg(gp(is_64,rt2)), Mem{rn_name(rn), imm7*scale(is_64)}], is_load) == llvm_mc(asm)
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
evidence: load_store.rs:516 encoding layout; encoder/mod.rs:496-497 dispatch; ARM ARM C6 LDNP/STNP signed offset
```

## encode_ldnp_stnp_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM field layout. Stronger differential already used on the same domain; this pins opc/101/V=0/000/L/imm7/Rt2/Rn/Rt independently of llvm-mc.
- Doc contract: load_store.rs:516 "Encoding: opc 101 V 000 L imm7 Rt2 Rn Rt" — asserted fingerprint fdc6a9e2
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_inv_arm_layout
- Formal: ∀ is_load, is_64, rt1, rt2, rn ∈ 0..31, imm7 ∈ [-64,63]. unpack(encode_ldnp_stnp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, imm7·scale}], is_load)) = (opc=10 if is_64 else 00, bits[29:27]=101, V=0, bits[25:23]=000, L=is_load, imm7[6:0], Rt2=rt2, Rn=rn, Rt=rt1)
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63 }
  body: unpack(word).opc == (is_64?2:0) && bits29_27==0b101 && V==0 && mode==0b000 && L==is_load && imm7_field==(imm7 as u7) && Rt2==rt2 && Rn==rn && Rt==rt1
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
evidence: ARM ARM C6 LDNP/STNP encoding; load_store.rs:516 word assembly; load_store.rs:531-533
```

## encode_ldnp_stnp_meta_fields
- Tier: 4
- Rationale: Metamorphic isolation: incrementing Rt1/Rt2/Rn/imm7 flips only that field; load XOR store is bit 22. Stronger round-trip rejected (no decoder).
- Doc contract: load_store.rs:516 "Encoding: opc 101 V 000 L imm7 Rt2 Rn Rt" — asserted fingerprint fdc6a9e2
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_meta_fields
- Formal: ∀ rt1∈0..30, rt2∈0..30, rn∈0..30, imm7∈[-64,62], is_64, is_load. encode(rt1+1) − encode(rt1) = 1; encode(rt2+1) − encode(rt2) = 1<<10; encode(rn+1) − encode(rn) = 1<<5; encode(imm7+1) isolates bits[21:15]; encode(load) XOR encode(store) = 1<<22
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_30, rt2: u32_0_30, rn: u32_0_30, imm7: i32_-64_62 }
  body: encode(rt1+1)-encode(rt1)==1 && encode(rt2+1)-encode(rt2)==(1<<10) && encode(rn+1)-encode(rn)==(1<<5) && (encode(load) XOR encode(store))==(1<<22)
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 30, type: u32 }
  rt2: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm7: { gen: int, min: -64, max: 62, type: i32 }
evidence: ARM ARM C6 LDNP/STNP field positions; load_store.rs:531-533
```

## encode_ldnp_stnp_neg_arity
- Tier: 3
- Rationale: Documented arity is 3 operands (body error "ldnp/stnp requires 3 operands"); ARM/llvm-mc/gas require a memory operand of signed-offset form only (no pre/post). Negative/error contract for too-few operands and a non-Mem third operand, including MemPreIndex/MemPostIndex which llvm-mc and gas reject.
- Doc contract: load_store.rs:515 "Encode LDNP/STNP (load/store pair non-temporal)" — asserted fingerprint 2a8ba6c0
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_neg_arity
- Formal: ∀ is_load, ops with |ops|<3 or ops[2] ∉ Mem. encode_ldnp_stnp(ops, is_load) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, kind]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, kind: non_mem_addr }
  body: encode_ldnp_stnp(too_few_or_non_mem, is_load).is_err()
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 9, type: u32 }
expected_error: String
evidence: load_store.rs:519-521 arity check; load_store.rs:538-539 unsupported operand; llvm-mc/gas reject pre/post writeback for LDNP/STNP
```

## encode_ldnp_stnp_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc and gas reject a fourth operand. The function accepts a slice with no upper bound (only `len < 3` errors), so extra operands stay in the domain. Body does not declare extra operands invalid.
- Doc contract: load_store.rs:515 "Encode LDNP/STNP (load/store pair non-temporal)" — asserted fingerprint 2a8ba6c0
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_neg_extra_operand
- Formal: ∀ is_load, is_64, rt1,rt2,rn ∈ 0..31, extra ∈ Operand. encode_ldnp_stnp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, 0}, extra], is_load) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: failing
- Counterexample: is_load=false, is_64=false, rt1=0, rt2=0, rn=0, extra=Reg("x0") → Ok(Word(0x28000000))
- Bug report: pbt-out/bug_reports/encode_ldnp_stnp_extra_operand.md

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, extra]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, extra: Operand }
  body: encode_ldnp_stnp([Reg, Reg, Mem, extra], is_load).is_err()
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: encoder/mod.rs:496-497 dispatch; llvm-mc/gas reject a fourth operand for ldnp/stnp
```

## encode_ldnp_stnp_neg_invalid_regs
- Tier: 3
- Rationale: ARM/llvm-mc/gas require Rt in {Wt,Xt,WZR,XZR} never SP; Rn in {Xn,SP} never XZR/W/WSP/x31; both Rt same width. Body does not declare these invalid; parse_reg_num aliases SP/XZR/W and discards Rt2 width.
- Doc contract: load_store.rs:515 "Encode LDNP/STNP (load/store pair non-temporal)" — asserted fingerprint 2a8ba6c0
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_neg_invalid_regs
- Formal: ∀ is_load, is_64, rt,rt2,rn ∈ 0..30. encode_ldnp_stnp of each of {SP as Rt1, SP as Rt2, XZR base, x31 base, W base, WSP base, mixed X/W pair} is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: failing
- Counterexample: is_load=false, is_64=false, rt=0, rt2=0, rn=0 → accepted ["SP as Rt1", "SP as Rt2", "XZR base", "x31 base", "W base", "WSP base", "mixed X/W pair"]
- Bug report: pbt-out/bug_reports/encode_ldnp_stnp_invalid_regs.md

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt, rt2, rn]
  domain: { is_load: bool, is_64: bool, rt: u32_0_30, rt2: u32_0_30, rn: u32_0_30 }
  body: encode(SP as Rt1).is_err() && encode(SP as Rt2).is_err() && encode(XZR base).is_err() && encode(x31 base).is_err() && encode(W base).is_err() && encode(WSP base).is_err() && encode(mixed X/W).is_err()
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rt2: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM C6 LDNP/STNP register classes; llvm-mc/gas reject SP dest, XZR/W/WSP/x31 base, mixed width
```

## encode_ldnp_stnp_neg_offset_range
- Tier: 3
- Rationale: ARM/llvm-mc require scaled signed imm7: W offset multiple of 4 in [-256,252]; X offset multiple of 8 in [-512,504]. Body shifts and masks rather than rejecting. Documented bounds sampled at bound±1, unaligned 1, i64::MIN/MAX.
- Doc contract: load_store.rs:516 "Encoding: opc 101 V 000 L imm7 Rt2 Rn Rt" — asserted fingerprint fdc6a9e2
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_neg_offset_range
- Formal: ∀ is_load, is_64, rt1,rt2,rn ∈ 0..31, off ∈ {min-1, max+1, 1, i64::MIN, i64::MAX}. encode_ldnp_stnp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, off}], is_load) is Err where min/max are the ARM range for that width
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: failing
- Counterexample: is_load=false, is_64=false, rt1=0, rt2=0, rn=0, which_off=0 (offset=-257) → Ok(Word(0x281F0000))
- Bug report: pbt-out/bug_reports/encode_ldnp_stnp_imm7_range.md

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, off]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, off: {min-1, max+1, 1, i64::MIN, i64::MAX} }
  body: encode_ldnp_stnp([Reg, Reg, Mem{off}], is_load).is_err()
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: ARM ARM C6 LDNP/STNP imm7 range; llvm-mc "index must be a multiple of 4/8 in range [-256,252]/[-512,504]"
```

## encode_ldnp_stnp_diff_simd_llvm_mc
- Tier: 5
- Rationale: README lists ldnp/stnp without restricting to integer; ARM and llvm-mc encode SIMD S/D/Q pairs (V=1). The function's own TODO admits it only handles V=0 — a documented limitation on an input get_reg accepts (s/d/q). Keep SIMD in the domain; disagreement is a bug with severity one step down.
- Doc contract: load_store.rs:517 "TODO: Only handles integer registers (V=0). FP/SIMD register support needed for V=1." — limitation fingerprint 10b6429a
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_diff_simd_llvm_mc
- Formal: ∀ is_load ∈ Bool, kind ∈ {s,d,q}, rt1,rt2,rn ∈ 0..31, imm7 ∈ [-64,63]. encode_ldnp_stnp([Reg(kind+rt1), Reg(kind+rt2), Mem{Xn|SP, imm7·scale(kind)}], is_load) = llvm-mc("ldnp/stnp Sk/Dk/Qk, …")
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: failing
- Counterexample: is_load=false, kind=0 (s), rt1=0, rt2=0, rn=0, imm7=-64 → stnp s0, s0, [x0, #-256] SUT=0x281F8000 llvm-mc=0x2C1F8000
- Bug report: pbt-out/bug_reports/encode_ldnp_stnp_simd.md

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, kind, rt1, rt2, rn, imm7]
  domain: { is_load: bool, kind: {s,d,q}, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63 }
  body: encode_ldnp_stnp([Reg(s/d/q), Reg(s/d/q), Mem], is_load) == llvm_mc(asm)
generators:
  is_load: { gen: bool }
  kind: { gen: int, min: 0, max: 2, type: u32 }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
evidence: load_store.rs:517 TODO admits V=1 unimplemented; ARM ARM C6 LDNP (SIMD&FP); llvm-mc encodes S/D/Q
```

## encode_ldnp_stnp_diff_alt_spellings
- Tier: 5
- Rationale: Sweep — uppercase / lr / w31 aliases are accepted by parse_reg_num and llvm-mc. Differential vs llvm-mc on those spellings. Stronger oracles already used on the canonical domain.
- Doc contract: load_store.rs:515 "Encode LDNP/STNP (load/store pair non-temporal)" — asserted fingerprint 2a8ba6c0
- Seed: encode_ldp_stp_pbt.rs encode_ldp_stp_diff_alt_spellings
- Formal: ∀ is_load ∈ Bool, which ∈ {uppercase X, wzr/W30/SP, lr-as-x30, w31-as-wzr}. encode_ldnp_stnp(alt spelling) = llvm-mc(canonical asm)
- Test file: src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldnp_stnp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, which]
  domain: { is_load: bool, which: u32_0_3 }
  body: encode_ldnp_stnp(alt_ops, is_load) == llvm_mc(canonical_asm)
generators:
  is_load: { gen: bool }
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: load_store.rs:281 parse_reg_num case-fold / lr / w31; llvm-mc accepts those aliases
```
