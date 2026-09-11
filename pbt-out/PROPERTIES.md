# Properties: encode_add_sub

Oracle classification (encode_add_sub):
- State Machine: rejected — pure function, no lifecycle/state.
- Differential: selected as strongest — assembler README states the builtin assembler "accepts the same textual assembly that GCC's gas would consume"; encoder module docs claim AArch64 32-bit encoding; llvm-mc is an independent same-job assembler. SUT-boundary: internal-helper of the assembler encoder. Mapping: (operands, is_sub, set_flags) ↔ GNU/LLVM assembly text.
- Algebraic round-trip: rejected — no in-tree ADD/SUB decoder.
- Algebraic idempotence: rejected — encoding is not a normalizer.
- Reference ARM ARM tables: weaker than live llvm-mc; used as field-layout evidence inside differential/negative properties.
- Crash-only: rejected — output words and documented Err strings are observable.

## encode_add_sub_diff_imm
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc on the documented GNU-style AArch64 ADD/SUB immediate encoding (imm12, auto-shift #N<<12, negative-imm alias). Stronger state machine rejected (no state). Round-trip rejected (no decoder).
- Seed: (none) — no existing encode_add_sub unit tests. Spec evidence: DESIGN_DOC.md "imm12 auto-shift"; data_processing.rs:326-350 comments "ADD Rd, Rn, #imm" / "Handle negative immediates" / auto-shift #4096.
- Formal: ∀ rd,rn ∈ GPR, is_64 ∈ Bool, is_sub ∈ Bool, set_flags ∈ Bool, imm ∈ ValidImm12Domain. (Rn=31 is SP/WSP, not XZR; Rd=31 is SP iff !set_flags else XZR). llvm_mc(asm) = w ⇒ encode_add_sub(...) = Ok(Word(w))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, imm, sh12]
  domain: { rd: gpr, rn: gpr, is_64: bool, is_sub: bool, set_flags: bool, imm: valid_imm12_or_shifted, sh12: bool }
  relation:
    op: eq
    lhs: encode_add_sub(ops, is_sub, set_flags)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  imm: { gen: int, min: -16773120, max: 16773120, type: i64 }
  sh12: { gen: bool }
evidence: src/backend/arm/assembler/README.md (GNU-style gas-compatible encoding); DESIGN_DOC.md imm12 auto-shift
```

## encode_add_sub_diff_shifted_reg
- Tier: 5
- Rationale: Differential vs llvm-mc for ADD/SUB shifted-register form (LSL/LSR/ASR). SP excluded here (different encoding class). ARM ARM C4 add/subtract (shifted register): sf op S 01011 shift 0 Rm imm6 Rn Rd.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..30}∪{xzr/wzr}, is_64, is_sub, set_flags, shift ∈ {lsl,lsr,asr}, amt ∈ [0, 63 if is_64 else 31]. encode_add_sub(...) = Ok(Word(llvm_mc(asm)))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, is_sub, set_flags, shift, amt]
  domain: { rd,rn,rm: gpr_not_sp, shift: {lsl,lsr,asr}, amt: 0..=(is_64?63:31) }
  relation:
    op: eq
    lhs: encode_add_sub(ops, is_sub, set_flags)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  shift: { gen: oneof, choices: ["lsl", "lsr", "asr"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM add/subtract (shifted register); llvm-mc same-job assembler
```

## encode_add_sub_diff_extended_and_sp
- Tier: 5
- Rationale: Differential vs llvm-mc for extended-register form and SP/WSP (must use extended form with UXTX/UXTW, not shifted-register which encodes 31 as XZR). Includes LSL #0..4 as UXTX alias when Rd or Rn is SP. Bounds 0 and 4 sampled exactly.
- Seed: (none). Doc evidence: data_processing.rs:390-402 "When Rn or Rd is SP ... we must use the extended register form with UXTX".
- Formal: ∀ valid extended/SP operand tuples. encode_add_sub(...) = Ok(Word(llvm_mc(asm)))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: add w0, wsp, w0, lsl #1 — SUT 0x0b0007e0 vs llvm-mc 0x0b2047e0
- Bug report: pbt-out/bug_reports/encode_add_sub_sp_lsl_shifted_form.md

```property
function: encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, is_sub, set_flags, ext, amt]
  domain: { ext: {uxtb,uxth,uxtw,uxtx,sxtb,sxth,sxtw,sxtx,lsl}, amt: 0..=4, SP allowed on rd/rn }
  relation:
    op: eq
    lhs: encode_add_sub(ops, is_sub, set_flags)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  ext: { gen: oneof, choices: ["uxtb","uxth","uxtw","uxtx","sxtb","sxth","sxtw","sxtx","lsl"] }
  amt: { gen: int, min: 0, max: 4, type: u32 }
evidence: data_processing.rs:390-402 SP must use extended form; ARM ARM add/subtract (extended register)
```

## encode_add_sub_diff_neon
- Tier: 5
- Rationale: encode_add_sub dispatches NEON vector ADD/SUB Vd.T,Vn.T,Vm.T when first operand is RegArrangement and set_flags=false. Differential vs llvm-mc. ADDS on NEON is invalid (negative sibling).
- Seed: (none)
- Formal: ∀ arr ∈ {8b,16b,4h,8h,2s,4s,2d}, vd,vn,vm ∈ 0..31, is_sub ∈ Bool. encode_add_sub([RegArrangement;3], is_sub, false) = Ok(Word(llvm_mc("add/sub Vd.arr, Vn.arr, Vm.arr")))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vn, vm, arr, is_sub]
  domain: { vd,vn,vm: 0..=31, arr: neon_arr, is_sub: bool }
  relation:
    op: eq
    lhs: encode_add_sub(ops, is_sub, false)
    rhs: llvm_mc(asm)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  vm: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, choices: ["8b","16b","4h","8h","2s","4s","2d"] }
  is_sub: { gen: bool }
evidence: data_processing.rs:296-300 NEON vector form; ARM ARM Advanced SIMD ADD/SUB
```

## encode_add_sub_neg_too_few_operands
- Tier: 4e
- Rationale: Documented error contract: "add/sub requires 3 operands". Negative/error oracle. Stronger differential does not apply to undersized operand lists (llvm-mc also rejects; we assert SUT's documented Err).
- Seed: (none). Evidence: data_processing.rs:292-294.
- Formal: ∀ ops, |ops| < 3, is_sub, set_flags. encode_add_sub(ops, is_sub, set_flags) = Err(e) ∧ e contains "requires 3 operands"
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_sub, set_flags]
  domain: { n: 0..=2 }
  relation:
    op: throws
    expr: encode_add_sub(ops_of_len_n, is_sub, set_flags)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
expected_error: String
evidence: data_processing.rs:292-294 add/sub requires 3 operands
```

## encode_add_sub_neg_imm_out_of_range
- Tier: 4e
- Rationale: ARM ADD/SUB immediate imm12 is 12 bits; shifted form requires low 12 bits zero and imm>>12 in 0..=0xFFF. llvm-mc rejects values outside that domain (range [0,4095] for the unshifted field). Field masking of overflow (explicit lsl #12 with imm>0xFFF) is a bug per pbt-patterns assembler range-contract rule. Bounds 4096, 4097, 0xFFF001, 0x1000000, i64::MIN sampled.
- Seed: (none). Evidence: data_processing.rs:348 "immediate {} does not fit in add/sub imm12 encoding"; llvm-mc "integer in range [0, 4095]"; DESIGN_DOC imm12.
- Formal: ∀ rd,rn,is_64,is_sub,set_flags, imm ∉ ValidImm12Domain (and not a valid auto-shift). encode_add_sub(...) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: Imm(4097) + Shift lsl #12 on w0,w1 — SUT Ok (masks to 1) vs expected Err
- Bug report: pbt-out/bug_reports/encode_add_sub_imm12_lsl12_mask.md

```property
function: encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, imm, explicit_lsl12]
  domain: { imm: invalid_imm12 }
  relation:
    op: throws
    expr: encode_add_sub(ops, is_sub, set_flags)
generators:
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  explicit_lsl12: { gen: bool }
expected_error: String
evidence: data_processing.rs:348 immediate does not fit in add/sub imm12 encoding
```

## encode_add_sub_neg_invalid_shift_extend
- Tier: 4e
- Rationale: ARM ARM: shifted-register imm6 in 0..=63 (sf=1) or 0..=31 (sf=0, imm6<5>==1 is UNALLOCATED); extend imm3 in 0..=4 (imm3>4 UNALLOCATED); ROR is not a valid ADD/SUB shift. llvm-mc rejects these. SUT must Err, not mask (& 0x3F / & 0x7) or default unknown shift to LSL.
- Seed: (none). Evidence: ARM ARM unallocated encodings; llvm-mc errors for lsl #64, w-reg lsl #32, sxtw #5, ror #N.
- Formal: ∀ invalid shift/extend (amt>max ∨ kind=ror ∨ extend_amt>4). encode_add_sub(...) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: [Reg(w0), Reg(w1), Reg(w2), Shift { kind: ror, amount: 0 }] — SUT Ok (defaults to LSL)
- Bug report: pbt-out/bug_reports/encode_add_sub_ror_accepted.md

```property
function: encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, amt, is_64]
  domain: { kind: shift_or_extend, amt: out_of_range }
  relation:
    op: throws
    expr: encode_add_sub(ops, is_sub, set_flags)
generators:
  kind: { gen: oneof, choices: ["lsl","lsr","asr","ror","sxtw","uxtx"] }
  amt: { gen: int, min: 0, max: 128, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: ARM ARM unallocated shifted/extended encodings
```

## encode_add_sub_metamorphic_neg_imm
- Tier: 4c
- Rationale: Documented assembler alias: add #-N encodes as sub #N and vice versa (when N is a valid imm12/auto-shift). Metamorphic: encode_add_sub(ops_with_-N, is_sub, s) = encode_add_sub(ops_with_N, !is_sub, s). Independent of llvm-mc; grounded in data_processing.rs:329-335 and llvm-mc `add x0,x1,#-1` → `sub x0,x1,#1`.
- Seed: (none)
- Formal: ∀ rd,rn,is_64,is_sub,set_flags, N ∈ ValidImm12Domain, N>0. encode_add_sub([Rd,Rn,Imm(-N)], is_sub, s) = encode_add_sub([Rd,Rn,Imm(N)], !is_sub, s)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_add_sub
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, n]
  domain: { n: valid positive imm12/auto-shift }
  relation:
    op: eq
    lhs: encode_add_sub([Rd,Rn,Imm(-n)], is_sub, set_flags)
    rhs: encode_add_sub([Rd,Rn,Imm(n)], !is_sub, set_flags)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  n: { gen: int, min: 1, max: 16773120, type: i64 }
evidence: data_processing.rs:329-335 Handle negative immediates add #-N to sub #N
```
