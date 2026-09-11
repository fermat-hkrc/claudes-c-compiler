# Properties: encode_add_sub

## encode_add_sub_diff_imm
- Tier: 2
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler). State machine rejected — encode_add_sub is a pure function with no lifecycle. Algebraic round-trip rejected — no in-tree ADD/SUB decoder. SUT-boundary: internal-helper of the GNU-style assembler; encode_instruction dispatches add/adds/sub/subs here. Argument mapping: (Rd,Rn,Imm,optional lsl#12,is_sub,set_flags) ↔ asm text. Shared contract: assembler README "accepts the same textual assembly that GCC's gas would consume" plus encoder "Encodes AArch64 instructions into 32-bit machine code words". Immediate form register 31 is SP/WSP (ARM ARM).
- Seed: DESIGN_DOC.md:338 (imm12 auto-shift); existing encode_add_sub_pbt::encode_add_sub_diff_imm
- Formal: ∀ rd,rn ∈ {0..31}, is_64,is_sub,set_flags ∈ bool, (imm,sh12) a valid imm12 or auto-shift (or negative alias). encode_add_sub([Reg(SP-or-GPR),Reg(SP-or-GPR),Imm(imm), optional Shift{lsl,12}], is_sub, set_flags) = Word(w) ∧ llvm-mc(asm) = w.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, imm, sh12]
  domain: { rd: 0..31, rn: 0..31, imm: valid_imm12_or_autoshift }
  relation:
    op: eq
    lhs: encode_add_sub([Reg(gpr_or_sp(is_64,rd)), Reg(gpr_or_sp(is_64,rn)), Imm(imm)] ++ lsl12(sh12), is_sub, set_flags)
    rhs: llvm_mc(asm_add_sub_imm(is_sub, set_flags, rd, rn, imm, sh12))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  imm: { gen: int, min: -4095, max: 16773120, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:1-7; DESIGN_DOC.md:338; ARM ARM ADD/SUB (immediate)
```

## encode_add_sub_diff_shifted_reg
- Tier: 2
- Rationale: Differential vs llvm-mc for the shifted-register form. Round-trip rejected (no decoder). State machine rejected (pure). Rd/Rn restricted to 0..30 so register 31 is not SP (shifted form encodes 31 as XZR). Shift kind in {lsl,lsr,asr}; amount in 0..=63 (sf=1) or 0..=31 (sf=0).
- Seed: existing encode_add_sub_pbt::encode_add_sub_diff_shifted_reg
- Formal: ∀ rd,rn ∈ {0..30}, rm ∈ {0..31}, is_64,is_sub,set_flags ∈ bool, kind ∈ {lsl,lsr,asr}, amt ∈ 0..max(sf). encode_add_sub([Reg,Reg,Reg,Shift{kind,amt}], is_sub, set_flags) = Word(w) ∧ llvm-mc(asm) = w.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, is_sub, set_flags, kind, amt]
  domain: { rd: 0..30, rn: 0..30, rm: 0..31, kind: {lsl,lsr,asr}, amt: 0..max_shift(is_64) }
  relation:
    op: eq
    lhs: encode_add_sub([Reg,Reg,Reg,Shift{kind,amt}], is_sub, set_flags)
    rhs: llvm_mc(asm_add_sub_shifted(is_sub, set_flags, rd, rn, rm, kind, amt))
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM ADD/SUB (shifted register); assembler README.md:5-14
```

## encode_add_sub_diff_extended_and_sp
- Tier: 2
- Rationale: Differential vs llvm-mc for extended-register form and the SP/WSP LSL alias (ARM ARM: when Rd or Rn is SP, the encoding is extended-register so 31 means SP not XZR; LSL #N with N in 0..=4 aliases UXTX/UXTW #N).
- Seed: existing encode_add_sub_pbt::encode_add_sub_diff_extended_and_sp
- Formal: ∀ valid extended-register or SP+LSL-alias operands accepted by llvm-mc. encode_add_sub(ops, is_sub, set_flags) = Word(w) ∧ llvm-mc(asm) = w.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_64=false, is_sub=false, set_flags=false, ext=uxtb, amt=1, use_lsl_alias=true (asm=add w0, wsp, w0, lsl #1; sut=0x0b0007e0 vs llvm-mc=0x0b2047e0)
- Bug report: pbt-out/bug_reports/encode_add_sub_sp_lsl_shifted_form.md

```property
function: encoder.data_processing.encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, is_sub, set_flags, ext, amt]
  domain: { rd: 0..31, rn: 0..31, rm: 0..30, ext: {uxtb,uxth,uxtw,uxtx,sxtb,sxth,sxtw,sxtx,lsl}, amt: 0..4 }
  relation:
    op: eq
    lhs: encode_add_sub(ops_extended_or_sp_lsl, is_sub, set_flags)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  amt: { gen: int, min: 0, max: 4, type: u32 }
evidence: ARM ARM ADD/SUB (extended register); llvm-mc add w0, wsp, w0, lsl #1
```

## encode_add_sub_diff_neon
- Tier: 2
- Rationale: Differential vs llvm-mc for NEON vector ADD/SUB Vd.T, Vn.T, Vm.T. encode_add_sub dispatches this form when the first operand is RegArrangement and set_flags is false. State machine / round-trip rejected as above.
- Seed: existing encode_add_sub_pbt::encode_add_sub_diff_neon; assembler README.md NEON three-same lists add/sub
- Formal: ∀ vd,vn,vm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, is_sub ∈ bool. encode_add_sub([RegArrangement(vN,T)]^3, is_sub, false) = Word(w) ∧ llvm-mc("add/sub vN.T, ...") = w.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vn, vm, arr, is_sub]
  domain: { vd: 0..31, vn: 0..31, vm: 0..31, arr: {8b,16b,4h,8h,2s,4s,2d} }
  relation:
    op: eq
    lhs: encode_add_sub([RegArrangement(v{vd},arr), RegArrangement(v{vn},arr), RegArrangement(v{vm},arr)], is_sub, false)
    rhs: llvm_mc(asm_neon_add_sub(is_sub, vd, vn, vm, arr))
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  vm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
evidence: assembler README.md:214 NEON three-same add/sub; encoder/data_processing.rs:305-310
```

## encode_add_sub_neg_too_few_operands
- Tier: 4e
- Rationale: Documented error path: encode_add_sub returns Err("add/sub requires 3 operands, got N") when len < 3. Stronger oracles do not apply to the invalid-arity domain. llvm-mc reports "too few operands".
- Seed: data_processing.rs:292-294; existing encode_add_sub_pbt::encode_add_sub_neg_too_few_operands
- Formal: ∀ ops with |ops| < 3, is_sub,set_flags ∈ bool. encode_add_sub(ops, is_sub, set_flags) = Err(s) ∧ s contains "requires 3 operands".
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_sub, set_flags]
  domain: { n: 0..2 }
  body: encode_add_sub(ops_of_len(n), is_sub, set_flags).is_err()
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
expected_error: String
evidence: data_processing.rs:292-294; llvm-mc too few operands
```

## encode_add_sub_neg_imm_out_of_range
- Tier: 4e
- Rationale: ARM ARM imm12 is 12 bits; with explicit lsl #12 the unshifted field must be in 0..=4095. llvm-mc: "integer in range [0, 4095]". SUT comment: "must fit in 12 bits". Values that are not a 12-bit unshifted or (N<<12) auto-shift must Err, not mask.
- Seed: data_processing.rs:323-330; existing encode_add_sub_pbt::encode_add_sub_neg_imm_out_of_range
- Formal: ∀ rd,rn ∈ {0..30}, is_64,is_sub,set_flags ∈ bool, imm not a valid imm12 encoding (or mag>0xFFF with explicit lsl#12). encode_add_sub([Reg,Reg,Imm(imm), optional Shift{lsl,12}], is_sub, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_64=false, is_sub=false, set_flags=false, imm=4097, explicit_lsl12=true
- Bug report: pbt-out/bug_reports/encode_add_sub_imm12_lsl12_mask.md

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, imm, explicit_lsl12]
  domain: { rd: 0..30, rn: 0..30 }
  body: encode_add_sub(imm_ops(rd, rn, imm, explicit_lsl12), is_sub, set_flags).is_err()
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  explicit_lsl12: { gen: bool }
expected_error: String
evidence: ARM ARM ADD/SUB (immediate) imm12 12-bit field; llvm-mc range [0,4095]; data_processing.rs:323 comment must fit in 12 bits
```

## encode_add_sub_neg_invalid_shift_extend
- Tier: 4e
- Rationale: ARM ARM shifted-register allows only LSL/LSR/ASR with imm6 in range; ROR is not a valid ADD/SUB shift. Extended-register imm3 > 4 is UNALLOCATED. llvm-mc and GNU as reject these. Encoder must Err, not default ROR to LSL or mask amounts.
- Seed: existing encode_add_sub_pbt::encode_add_sub_neg_invalid_shift_extend; llvm-mc `add w0, w1, w2, ror #0`
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64,is_sub,set_flags ∈ bool, invalid ∈ {ROR, shift_amt out of range, extend_amt > 4}. encode_add_sub([Reg,Reg,Reg,invalid], is_sub, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_64=false, is_sub=false, set_flags=false, class=0, extra=0 (ROR #0)
- Bug report: pbt-out/bug_reports/encode_add_sub_ror_accepted.md

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, is_sub, set_flags, class, extra]
  body: encode_add_sub(bad_shift_or_extend_ops(rd, rn, rm, class, extra, is_64), is_sub, set_flags).is_err()
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  class: { gen: int, min: 0, max: 3, type: u8 }
  extra: { gen: int, min: 0, max: 64, type: u32 }
expected_error: String
evidence: ARM ARM ADD/SUB shifted-register shift in LSL LSR ASR; llvm-mc rejects ror; ARM ARM extended imm3 in 0..=4
```

## encode_add_sub_metamorphic_neg_imm
- Tier: 4c
- Rationale: SUT comment and ARM assembler alias: add #-N encodes as sub #N and vice versa. Metamorphic relation independent of llvm-mc. Differential is stronger and is a sibling property; this pins the alias without an external tool. Round-trip rejected (no decoder).
- Seed: data_processing.rs:311-317 "Handle negative immediates: add #-N -> sub #N and vice versa"; existing encode_add_sub_pbt::encode_add_sub_metamorphic_neg_imm
- Formal: ∀ rd ∈ {0..30}, rn ∈ {0..31}, is_64,is_sub,set_flags ∈ bool, n a positive valid imm12 magnitude. encode_add_sub([Rd,Rn,Imm(-n)], is_sub, s) = encode_add_sub([Rd,Rn,Imm(n)], !is_sub, s).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, n]
  domain: { n: positive valid imm12 magnitude }
  relation:
    op: eq
    lhs: encode_add_sub([Rd,Rn,Imm(-n)], is_sub, set_flags)
    rhs: encode_add_sub([Rd,Rn,Imm(n)], not is_sub, set_flags)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 1, max: 16773120, type: i64 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
evidence: data_processing.rs:311-317; ARM assembler negative-imm alias
```

## encode_add_sub_neg_imm_bad_shift
- Tier: 4e
- Rationale: ARM ARM ADD/SUB (immediate) permits only LSL #0 or LSL #12 after the immediate. llvm-mc: "only 'lsl #+N' valid after immediate" for lsr/asr/ror, and rejects lsl #N for N ∉ {0,12}. Silent ignore of a fourth operand would assemble a different instruction than the source text. Gas-compatible assembler contract. Stronger oracles do not apply to this invalid domain.
- Seed: llvm-mc `add x0, x1, #1, lsr #12` → error: only 'lsl #+N' valid after immediate
- Formal: ∀ rd,rn ∈ {0..30}, is_64,is_sub,set_flags ∈ bool, imm ∈ 0..0xFFF, (kind,amt) with kind ∈ {lsr,asr,ror} ∨ (kind=lsl ∧ amt ∉ {0,12}). encode_add_sub([Reg,Reg,Imm(imm),Shift{kind,amt}], is_sub, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_64=false, is_sub=false, set_flags=false, imm=0, class=0, amt=0 (lsr #0)
- Bug report: pbt-out/bug_reports/encode_add_sub_imm_bad_shift_ignored.md

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, imm, kind, amt]
  domain: { rd: 0..30, rn: 0..30, imm: 0..4095, amt: 0..63 }
  body: encode_add_sub([Reg,Reg,Imm(imm),Shift{kind,amt}], is_sub, set_flags).is_err()
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: int, min: 0, max: 4095, type: i64 }
  amt: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: ARM ARM ADD/SUB (immediate) sh in 0 or 1 equals LSL 0 or 12; llvm-mc only lsl valid after immediate; assembler README.md:5-14
```

## encode_add_sub_neg_mixed_width
- Tier: 4e
- Rationale: ARM ARM ADD/SUB immediate and shifted-register forms require all registers the same width (all W or all X). llvm-mc rejects `add x0, w1, #1` and `add w0, x1, x2`. A single sf bit taken only from Rd would silently encode the wrong-width instruction. Extended-register Wm with Xd is a different form and is excluded from this domain. Negative/error.
- Seed: llvm-mc `add x0, w1, #1` → error: invalid operand
- Formal: ∀ rd,rn ∈ {0..30}, rd64 ≠ rn64, is_sub,set_flags ∈ bool, imm ∈ 0..0xFFF. encode_add_sub([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(imm)], is_sub, set_flags) = Err(_). And ∀ rd,rn,rm ∈ {0..30} with not-all-equal widths, no extend: encode_add_sub([Reg,Reg,Reg], is_sub, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=true, rn64=false, rm64=false, is_sub=false, set_flags=false, use_imm=false (ops=[x0, w0, w0])
- Bug report: pbt-out/bug_reports/encode_add_sub_mixed_width.md

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64, is_sub, set_flags, use_imm, imm]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, imm: 0..4095 }
  body: encode_add_sub(mixed_width_ops(rd, rn, rm, rd64, rn64, rm64, use_imm, imm), is_sub, set_flags).is_err()
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: int, min: 0, max: 4095, type: i64 }
expected_error: String
evidence: ARM ARM ADD Wd Wn imm or Xd Xn imm; llvm-mc mixed-width error; assembler README.md:5-14
```

## encode_add_sub_reloc_lo12
- Tier: 4d
- Rationale: RelocType::AddAbsLo12 is documented as R_AARCH64_ADD_ABS_LO12_NC for ADD :lo12:. DESIGN_DOC.md:350 lists ADD_ABS_LO12_NC. encode_add_sub must return WordWithReloc with that type, the given symbol, addend 0 (Modifier) or the given offset (ModifierOffset), and an ADD-immediate word with imm12=0 (reloc fills it). Differential vs llvm-mc encoding of the unresolved instruction is a sibling check on the word bits. State machine / round-trip rejected as above.
- Seed: encoder/mod.rs:55-56 RelocType::AddAbsLo12; DESIGN_DOC.md:350; data_processing.rs:345-356
- Formal: ∀ rd,rn ∈ {0..30}, is_64,is_sub,set_flags ∈ bool, sym a nonempty ident, off ∈ i64. encode_add_sub([Reg,Reg,Modifier{lo12,sym}], ...) = WordWithReloc { word = (sf<<31)|(op<<30)|(s<<29)|(0b10001<<24)|(rn<<5)|rd, reloc_type=AddAbsLo12, symbol=sym, addend=0 }. ModifierOffset{lo12,sym,off} same with addend=off.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, sym, off]
  domain: { rd: 0..30, rn: 0..30 }
  body: let WordWithReloc { word, reloc } = encode_add_sub([Reg,Reg,Modifier{lo12,sym} or ModifierOffset], is_sub, set_flags) in reloc.reloc_type == AddAbsLo12 && reloc.symbol == sym && reloc.addend == off && (word & 0x1F) == rd && ((word>>5)&0x1F) == rn && ((word>>31)&1)==sf && ((word>>30)&1)==op && ((word>>29)&1)==s && ((word>>24)&0x1F)==0b10001 && ((word>>10)&0xFFF)==0
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  off: { gen: int, min: -4096, max: 4096, type: i64 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
evidence: encoder/mod.rs:55-56; DESIGN_DOC.md:350; data_processing.rs:345-390
```

## encode_add_sub_neg_fp_reg
- Tier: 4e
- Rationale: ADD/SUB data-processing form uses W/X registers (assembler README Data Processing vs FP/NEON). llvm-mc rejects `adds d0, d1, d2`. parse_reg_num accepts d/s/q/v/h/b prefixes, so this path was untested. Coverage-sweep: FP register names at any of the three GPR slots must Err. Dispatch filters scalar d-regs only for add/sub, not adds/subs, so this is caller-reachable.
- Seed: llvm-mc `adds d0, d1, d2` → error: invalid operand; assembler README.md:214
- Formal: ∀ which ∈ {0,1,2}, is_sub,set_flags ∈ bool, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_add_sub(ops with ops[which]=Reg(prefix||n) and other slots GPR, is_sub, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_sub=false, set_flags=false, prefix=d, n=0 (ops=[d0, x1, x2])
- Bug report: pbt-out/bug_reports/encode_add_sub_fp_reg.md

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_sub, set_flags, prefix, n]
  domain: { which: 0..2, n: 0..31 }
  body: encode_add_sub(ops_with_fp_at(which, prefix, n), is_sub, set_flags).is_err()
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
expected_error: String
evidence: assembler README.md:214 Data Processing GPR vs FP/NEON; ARM ARM ADD Wd/Xd; llvm-mc rejects d/s/v forms; encoder/mod.rs:226 adds dispatches without neon filter
```

## encode_add_sub_neg_adds_sp_rd
- Tier: 4e
- Rationale: ARM ARM ADDS/SUBS (immediate) Rd is Wd/Xd, not SP/WSP. llvm-mc rejects `adds sp, x0, #0`. Encoding SP as 31 with S=1 silently assembles ADDS XZR (i.e. CMP), a different instruction. Coverage-sweep of the set_flags && Rd==SP path.
- Seed: llvm-mc `adds sp, x0, #0` → error: invalid operand
- Formal: ∀ is_64,is_sub ∈ bool, rn ∈ {0..30}, imm ∈ 0..0xFFF. encode_add_sub([Reg(sp/wsp), Reg(gpr), Imm(imm)], is_sub, true) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: is_64=false, is_sub=false, rn=0, imm=0 (ops=[wsp, w0, #0], set_flags=true)
- Bug report: pbt-out/bug_reports/encode_add_sub_adds_sp_rd.md

```property
function: encoder.data_processing.encode_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, is_sub, rn, imm]
  domain: { rn: 0..30, imm: 0..4095 }
  body: encode_add_sub([Reg(sp_or_wsp), Reg(gpr), Imm(imm)], is_sub, true).is_err()
generators:
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: int, min: 0, max: 4095, type: i64 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
expected_error: String
evidence: ARM ARM ADDS/SUBS Rd is Wd/Xd not SP; llvm-mc rejects adds sp; assembler README.md:5-14
```

## encode_add_sub_reloc_tprel
- Tier: 4d
- Rationale: RelocType documents TlsLeAddTprelLo12 (R_AARCH64_TLSLE_ADD_TPREL_LO12_NC) and TlsLeAddTprelHi12 (R_AARCH64_TLSLE_ADD_TPREL_HI12). Coverage-sweep of the tprel modifier arms. tprel_hi12 must set sh=1 (bit 22). State machine / round-trip rejected as above.
- Seed: encoder/mod.rs:72-74; data_processing.rs:357-375
- Formal: ∀ rd,rn ∈ {0..30}, is_64,is_sub,set_flags ∈ bool, hi ∈ bool, sym ident. encode_add_sub([Reg,Reg,Modifier{tprel_hi12|tprel_lo12_nc,sym}], ...) = WordWithReloc { reloc_type = TlsLeAddTprelHi12 if hi else TlsLeAddTprelLo12, symbol=sym, addend=0, word sh bit = hi }.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_add_sub
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, is_sub, set_flags, hi, suffix]
  domain: { rd: 0..30, rn: 0..30 }
  body: let WordWithReloc { word, reloc } = encode_add_sub([Reg,Reg,Modifier{tprel,sym}], is_sub, set_flags) in reloc.symbol == sym && reloc.addend == 0 && ((word >> 22) & 1) == hi && reloc_type matches hi
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  is_sub: { gen: bool }
  set_flags: { gen: bool }
  hi: { gen: bool }
evidence: encoder/mod.rs:72-74; data_processing.rs:357-375
```
