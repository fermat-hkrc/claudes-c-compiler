# Properties: encode_sbc

## encode_sbc_diff_gpr_same_width
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler of the same GNU-style SBC/SBCS text). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree SBC decoder. encode_adc rejected (same-job gate: ADC / op=0). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping [Reg(Rd),Reg(Rn),Reg(Rm)]+set_flags <-> `sbc`/`sbcs` Rd, Rn, Rm.
- Seed: src/backend/arm/codegen/i128_ops.rs:73 emits `sbc x1, x3, x5`
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, set_flags ∈ Bool. encode_sbc([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))], set_flags) = Word(v) ∧ llvm-mc(-triple=aarch64, asm) = v where asm is `sbc`/`sbcs` with matching X/W names (31 = xzr/wzr)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, set_flags: bool }
  relation:
    op: eq
    lhs: encode_sbc(ops, set_flags)
    rhs: llvm_mc_word(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
evidence: "README.md:14 gas-compat; README.md:214 sbc/sbcs; encoder/mod.rs:283-284 dispatch; ARM ARM SBC sf op=1 S 11010000 Rm 000000 Rn Rd"
```

## encode_sbc_meta_s_bit
- Tier: 4
- Rationale: ARM ARM S bit at 29 distinguishes SBC (S=0) from SBCS (S=1); otherwise identical. Metamorphic: XOR of the two encodings equals 1<<29. Differential is stronger on the X/W domain; this pins the flag bit independently of llvm-mc.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool. encode_sbc(ops, false) XOR encode_sbc(ops, true) = 1<<29
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_sbc(ops, false) XOR encode_sbc(ops, true)
    rhs: 1u32 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: "ARM ARM S bit at 29; data_processing.rs:788-790; encoder/mod.rs:283-284 sbc vs sbcs"
```

## encode_sbc_meta_vs_adc
- Tier: 4
- Rationale: ARM ARM Add/subtract (with carry): SBC op=1 vs ADC op=0, otherwise identical (same sf, S, opcode 11010000, Rm, Rn, Rd). For the same operands, SBC XOR ADC = bit 30. encode_adc is a different-job sibling used only as a metamorphic companion, not a differential reference.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, set_flags ∈ Bool. encode_sbc(ops, set_flags) XOR encode_adc(ops, set_flags) = 1<<30
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, set_flags: bool }
  relation:
    op: eq
    lhs: encode_sbc(ops, set_flags) XOR encode_adc(ops, set_flags)
    rhs: 1u32 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
evidence: "data_processing.rs:778 ADC no bit30; data_processing.rs:790 SBC sets bit30; ARM ARM op ADC=0 SBC=1"
```

## encode_sbc_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM: sf at 31, op=1 at 30, S at 29, bits[28:21]=11010000, Rm at [20:16], bits[15:10]=000000, Rn at [9:5], Rd at [4:0]. Differential is stronger on the GPR domain; this pins the field layout independently of llvm-mc.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, set_flags ∈ Bool. encode_sbc(ops, set_flags) = Word(w) ⇒ (w&0x1F)=rd ∧ ((w>>5)&0x1F)=rn ∧ ((w>>16)&0x1F)=rm ∧ ((w>>31)&1)=sf ∧ ((w>>29)&1)=S ∧ ((w>>30)&1)=1 ∧ ((w>>21)&0xFF)=0b11010000 ∧ ((w>>10)&0x3F)=0
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, set_flags: bool }
  relation:
    op: eq
    lhs: encode_sbc(ops, set_flags)
    rhs: Word((sf<<31)|(1<<30)|(s<<29)|(0b11010000<<21)|(rm<<16)|(rn<<5)|rd)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
evidence: "ARM ARM SBC sf 1 S 11010000 Rm 000000 Rn Rd; data_processing.rs:790"
```

## encode_sbc_meta_ngc_alias
- Tier: 4
- Rationale: ARM ARM / GNU as / llvm-mc alias: NGC Rd, Rm encodes as SBC Rd, ZR, Rm (and NGCS as SBCS Rd, ZR, Rm). Metamorphic equality of encode_sbc(Rd, ZR, Rm) with llvm-mc `ngc`/`ngcs`.
- Seed: (none)
- Formal: ∀ rd,rm ∈ {0..31}, is_64 ∈ Bool, set_flags ∈ Bool. encode_sbc([Reg(Rd), Reg(ZR), Reg(Rm)], set_flags) = llvm-mc(`ngc`/`ngcs` Rd, Rm)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, set_flags]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, set_flags: bool }
  relation:
    op: eq
    lhs: encode_sbc([Reg(Rd), Reg(ZR), Reg(Rm)], set_flags)
    rhs: llvm_mc_word(ngc_asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
evidence: "ARM ARM NGC alias of SBC Rd ZR Rm; llvm-mc sbc x0 xzr x1 equals ngc x0 x1"
```

## encode_sbc_meta_lr_alias
- Tier: 4
- Rationale: Coverage sweep of the documented lr alias. parse_reg_num maps lr to 30; is_64bit_reg treats lr as X. llvm-mc `sbc lr, x0, x1` equals `sbc x30, x0, x1`. Metamorphic plus differential.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, a,b ∈ {0..30}, set_flags ∈ Bool. encode_sbc(ops with slot which = lr) = encode_sbc(ops with slot which = x30) = llvm-mc(asm with lr)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [which, a, b, set_flags]
  domain: { which: 0..2, a: 0..30, b: 0..30, set_flags: bool }
  relation:
    op: eq
    lhs: encode_sbc(ops_lr, set_flags)
    rhs: encode_sbc(ops_x30, set_flags)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  set_flags: { gen: bool }
evidence: "parse_reg_num encoder/mod.rs:136 lr maps to 30; llvm-mc sbc lr equals sbc x30"
```

## encode_sbc_neg_too_few_operands
- Tier: 4
- Rationale: ARM ARM SBC takes three registers. llvm-mc rejects `sbc x0, x1` (too few operands). README.md:14 gas-compat. Must Err.
- Seed: (none)
- Formal: ∀ n ∈ {0,1,2}, set_flags ∈ Bool. encode_sbc(ops[:n], set_flags) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, set_flags]
  domain: { n: 0..2, set_flags: bool }
  relation:
    op: throws
    expr: encode_sbc(ops[:n], set_flags)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  set_flags: { gen: bool }
expected_error: String
evidence: "llvm-mc rejects sbc x0 x1 too few operands; ARM ARM three GPRs; get_reg encoder/mod.rs:956"
```

## encode_sbc_neg_non_register
- Tier: 4
- Rationale: SBC operands are GPRs. Imm/Symbol/Mem/Shift/Cond at any of the three slots are not valid (llvm-mc / ARM ARM). Must Err.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, bad ∈ {Imm, Symbol, Mem, Shift, Cond}. encode_sbc(ops with slot `which` = bad, set_flags) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad, set_flags]
  domain: { which: 0..2, bad: non-Reg Operand, set_flags: bool }
  relation:
    op: throws
    expr: encode_sbc(ops, set_flags)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  set_flags: { gen: bool }
expected_error: String
evidence: "ARM ARM SBC operands are GPRs; get_reg encoder/mod.rs:956 expected register; llvm-mc rejects non-GPR"
```

## encode_sbc_neg_invalid_reg_name
- Tier: 4
- Rationale: Names that parse_reg_num rejects (x32, w32, foo, empty, r0, x, x-1, x99) must Err with invalid register.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, name ∈ {x32, w32, x99, w99, "", foo, r0, x, x-1}. encode_sbc(ops with slot `which` = Reg(name)) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name, set_flags]
  domain: { which: 0..2, name: invalid GPR names, set_flags: bool }
  relation:
    op: throws
    expr: encode_sbc(ops, set_flags)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  set_flags: { gen: bool }
expected_error: String
evidence: "get_reg encoder/mod.rs:956-961 parse_reg_num None; ARM ARM SBC Rd Rn Rm are GPR 0-31"
```

## encode_sbc_neg_fp_reg
- Tier: 4
- Rationale: ARM ARM SBC Rd/Rn/Rm are GPRs. llvm-mc rejects `sbc d0, x1, x2`. Dedicated generator over {d,s,q,v,h,b} so a failure shrinks to an FP witness.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_sbc(ops with slot `which` = Reg(prefix||n)) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, set_flags=false, prefix="d", n=0 (sbc d0, x1, x2 encodes as sbc w0, w1, w2 / 0x5a020020)
- Bug report: pbt-out/bug_reports/encode_sbc_fp_reg.md

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n, set_flags]
  domain: { which: 0..2, prefix: FP/SIMD prefix, n: 0..31, set_flags: bool }
  relation:
    op: throws
    expr: encode_sbc(ops, set_flags)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  set_flags: { gen: bool }
expected_error: String
evidence: "ARM ARM SBC GPRs; llvm-mc rejects sbc d0; parse_reg_num encoder/mod.rs:141 accepts d s q v h b"
```

## encode_sbc_neg_extra_shift
- Tier: 4
- Rationale: ARM ARM Add/subtract (with carry) has no shift field (bits 15:10 fixed 000000). llvm-mc rejects `sbc x0, x1, x2, lsl #0`. Extra operand must Err.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ Bool, set_flags ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ {0..63}. encode_sbc([Rd,Rn,Rm,Shift(kind,amt)], set_flags) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_64=false, set_flags=false, kind="lsl", amt=0 (sbc w0, w0, w0, lsl #0 encodes as sbc w0, w0, w0 / 0x5a000000)
- Bug report: pbt-out/bug_reports/encode_sbc_extra_shift_ignored.md

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags, kind, amt]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, is_64: bool, set_flags: bool, kind: shift kind, amt: 0..63 }
  relation:
    op: throws
    expr: encode_sbc([Rd, Rn, Rm, Shift], set_flags)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
  amt: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: "llvm-mc rejects sbc with lsl; ARM ARM bits 15-10 fixed 000000; README.md:14 gas-compat"
```

## encode_sbc_neg_mixed_width
- Tier: 4
- Rationale: ARM ARM Rd/Rn/Rm must be the same width (all X or all W). llvm-mc rejects `sbc w0, w1, x2`. Mixed X/W must Err.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..30}, rd64,rn64,rm64 ∈ Bool not all equal, set_flags ∈ Bool. encode_sbc([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)], set_flags) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true, set_flags=false (sbc w0, w0, x0 encodes as sbc w0, w0, w0 / 0x5a000000)
- Bug report: pbt-out/bug_reports/encode_sbc_mixed_width.md

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64, set_flags]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, mixed widths, set_flags: bool }
  relation:
    op: throws
    expr: encode_sbc(ops, set_flags)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
  set_flags: { gen: bool }
expected_error: String
evidence: "ARM ARM SBC same width; llvm-mc rejects mixed X W; README.md:14 gas-compat"
```

## encode_sbc_neg_sp
- Tier: 4
- Rationale: ARM ARM register 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `sbc sp, x0, x1`. SP/WSP at any slot must Err.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, is_64 ∈ Bool, set_flags ∈ Bool. encode_sbc(ops with slot `which` = SP/WSP) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, set_flags=false, a=0, b=0 (sbc wsp, w0, w0 encodes as sbc wzr, w0, w0 / 0x5a00001f)
- Bug report: pbt-out/bug_reports/encode_sbc_sp_as_zr.md

```property
function: encode_sbc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, set_flags]
  domain: { which: 0..2, is_64: bool, set_flags: bool }
  relation:
    op: throws
    expr: encode_sbc(ops, set_flags)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
expected_error: String
evidence: "ARM ARM register 31 is ZR never SP; llvm-mc rejects sbc sp; parse_reg_num encoder/mod.rs:134 maps sp to 31"
```
