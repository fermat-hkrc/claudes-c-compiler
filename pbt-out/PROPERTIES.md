# Properties: encode_crc32

## encode_crc32_diff_valid_gpr
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler, same GNU-style text). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree CRC32 decoder). encode_clz/encode_cls rejected as same-job siblings (Data-processing 2-source, not CRC). ARM ARM field unpack is a weaker invariant used in encode_crc32_arm_fields.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: src/backend/arm/assembler/encoder/bitfield.rs encode_bfi_pbt llvm-mc differential
- Formal: ∀ mnemonic ∈ {crc32b,crc32h,crc32w,crc32x,crc32cb,crc32ch,crc32cw,crc32cx}, rd,rn,rm ∈ 0..31. encode_crc32(mnemonic, [Wd(rd), Wn(rn), Wm|Xm(rm)]) = llvm-mc(mnemonic Wd, Wn, Wm|Xm) where Wm|Xm is W iff mnemonic ∈ {crc32b,crc32h,crc32w,crc32cb,crc32ch,crc32cw} else X
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm]
  domain: { mnemonic: eight CRC32 mnemonics, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_crc32(mnemonic, [Wd(rd), Wn(rn), W_or_X(mnemonic, rm)])
    rhs: llvm_mc(mnemonic, Wd(rd), Wn(rn), W_or_X(mnemonic, rm))
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:12 gas-compat; encoder/mod.rs:1047-1048 dispatch; llvm-mc -triple=aarch64 -mattr=+crc
```

## encode_crc32_arm_fields
- Tier: 4
- Rationale: ARM ARM CRC32 layout is an exact structural invariant independent of llvm-mc. Weaker than differential; kept as a second oracle so a llvm-mc mapping bug cannot hide a field-layout error. Round-trip rejected (no decoder).
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: bitfield.rs encode_bfi_pbt ARM field unpack
- Formal: ∀ mnemonic ∈ CRC32_8, rd,rn,rm ∈ 0..31. let w = encode_crc32(mnemonic, valid_ops). w[31]=sf(mnemonic) ∧ w[30:21]=0011010110 ∧ w[20:16]=rm ∧ w[15:13]=010 ∧ w[12]=C(mnemonic) ∧ w[11:10]=sz(mnemonic) ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm]
  domain: { mnemonic: eight CRC32 mnemonics, rd: 0..31, rn: 0..31, rm: 0..31 }
  body: fields(encode_crc32(mnemonic, valid_ops)) match ARM CRC32 layout for (sf, C, sz, rd, rn, rm)
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: bitfield.rs:243 encoding comment; ARM ARM CRC32/CRC32C
```

## encode_crc32_meta_c_sz
- Tier: 4
- Rationale: Metamorphic: CRC32C vs CRC32 of the same size differs only in bit 12 (C); B/H/W/X of the same polynomial differ only in sf and sz. Independent of llvm-mc. Round-trip rejected (no decoder).
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt metamorphic Rd/Rn isolation
- Formal: ∀ sz ∈ {b,h,w,x}, rd,rn,rm ∈ 0..31. encode_crc32(crc32c∥sz, ops) XOR encode_crc32(crc32∥sz, ops) = 1<<12. ∀ poly ∈ {crc32,crc32c}. encodings of b/h/w/x with identical regs differ only in bits {31,11,10}
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [sz, rd, rn, rm]
  domain: { sz: {b,h,w,x}, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_crc32("crc32c"+sz, ops) XOR encode_crc32("crc32"+sz, ops)
    rhs: 1<<12
generators:
  sz: { gen: oneof, items: ["b", "h", "w", "x"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: bitfield.rs:243 C and sz fields; ARM ARM CRC32 vs CRC32C
```

## encode_crc32_meta_rd_rn_rm
- Tier: 4
- Rationale: Metamorphic field isolation: incrementing Rd/Rn/Rm by 1 updates only that 5-bit field. Independent of llvm-mc.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt metamorphic Rd/Rn
- Formal: ∀ mnemonic ∈ CRC32_8, rd,rn,rm ∈ 0..30. encode_crc32(..., rd+1, ...) differs from base only in bits[4:0]; rn+1 only in bits[9:5]; rm+1 only in bits[20:16]
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm]
  domain: { mnemonic: eight CRC32 mnemonics, rd: 0..30, rn: 0..30, rm: 0..30 }
  body: mutating one of rd/rn/rm by +1 flips only that 5-bit field
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
evidence: bitfield.rs:243 Rd/Rn/Rm field positions
```

## encode_crc32_neg_arity
- Tier: 4
- Rationale: llvm-mc / gas reject CRC32 with fewer than 3 operands ("too few operands"). get_reg on a missing slot returns Err, which is the documented assembler contract. Extra-operand case is a separate property (encode_crc32_neg_extra_operand) because the body has no upper bound.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_arity
- Formal: ∀ mnemonic ∈ CRC32_8, n ∈ 0..2, ops with n registers. encode_crc32(mnemonic, ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, n]
  domain: { mnemonic: eight CRC32 mnemonics, n: 0..2 }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, ops_of_len(n)).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  n: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; get_reg missing-slot Err
```

## encode_crc32_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc / gas reject a 4th CRC32 operand ("invalid operand"). README.md:12 gas-compat. Body has no operands.len() check so extra operands are ignored — that is the candidate bug, not a reason to shrink the domain.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_extra_operand
- Formal: ∀ mnemonic ∈ CRC32_8, valid 3-operand CRC32 ops, extra ∈ Operand. encode_crc32(mnemonic, ops++[extra]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: failing
- Counterexample: encode_crc32("crc32b", [Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")]) → Ok(Word(0x1ac04000))
- Bug report: pbt-out/bug_reports/encode_crc32_extra_operand.md

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm, extra]
  domain: { mnemonic: eight CRC32 mnemonics, rd: 0..31, rn: 0..31, rm: 0..31, extra: Operand }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, valid_ops ++ [extra]).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: ["Reg(x0)", "Imm(0)", "Shift(lsl,0)"] }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on 4th operand; README.md:12
```

## encode_crc32_neg_sp
- Tier: 4
- Rationale: ARM CRC32 uses ZR not SP at register 31; llvm-mc rejects sp/wsp in any CRC32 slot. parse_reg_num maps both to 31; the body does not distinguish them. Domain stays the full SP/WSP set.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_sp
- Formal: ∀ mnemonic ∈ CRC32_8, slot ∈ {0,1,2}, sp ∈ {sp,wsp}. encode_crc32(mnemonic, ops with slot=sp) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: failing
- Counterexample: encode_crc32("crc32b", [Reg("wsp"), Reg("w0"), Reg("w0")]) → Ok(Word) encoding Rd=31 as WZR
- Bug report: pbt-out/bug_reports/encode_crc32_sp_as_zr.md

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, slot, sp]
  domain: { mnemonic: eight CRC32 mnemonics, slot: 0..2, sp: {sp, wsp} }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, ops_with_sp_at(slot)).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  slot: { gen: int, min: 0, max: 2, type: u32 }
  sp: { gen: oneof, items: ["sp", "wsp"] }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on sp/wsp; ARM Rd/Rn/Rm are ZR not SP
```

## encode_crc32_neg_wrong_width
- Tier: 4
- Rationale: ARM and llvm-mc require Wd, Wn for every CRC32 form and Wm (B/H/W) or Xm (X). Mixed/wrong-width triples (X as Rd/Rn, W as Rm of crc32x, X as Rm of crc32b) are rejected by llvm-mc. Body discards get_reg's is_64 flag.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_mixed_width
- Formal: ∀ mnemonic ∈ CRC32_8, rd,rn,rm ∈ 0..31, widths that violate (Rd=W ∧ Rn=W ∧ Rm=W_if_BH W_else_X). encode_crc32(mnemonic, ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: failing
- Counterexample: encode_crc32("crc32b", [Reg("w0"), Reg("w0"), Reg("x0")]) → Ok(Word) (Rm must be W)
- Bug report: pbt-out/bug_reports/encode_crc32_wrong_width.md

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm, rd64, rn64, rm64]
  domain: { mnemonic: eight CRC32 mnemonics, rd: 0..31, rn: 0..31, rm: 0..31, widths: not the ARM-required shape }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, mixed_width_ops).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on X-as-Wd/Wn or W-as-Xm; ARM CRC32 register shape
```

## encode_crc32_neg_fp
- Tier: 4
- Rationale: Sweep — llvm-mc rejects FP/SIMD prefixes (d/s/q/v/h/b) as CRC32 operands. parse_reg_num accepts those prefixes; the body never checks is_fp_reg. Documented by README.md:12 gas-compat.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_fp
- Formal: ∀ mnemonic ∈ CRC32_8, slot ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_crc32(mnemonic, ops with slot=prefix∥n) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: failing
- Counterexample: encode_crc32("crc32b", [Reg("d0"), Reg("w1"), Reg("w2")]) → Ok(Word(0x1ac24020))
- Bug report: pbt-out/bug_reports/encode_crc32_fp_as_gpr.md

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, slot, prefix, n]
  domain: { mnemonic: eight CRC32 mnemonics, slot: 0..2, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, ops_with_fp_at(slot)).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  slot: { gen: int, min: 0, max: 2, type: u32 }
  prefix: { gen: oneof, items: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on d/s/q/v/h/b; README.md:12
```

## encode_crc32_neg_invalid_name
- Tier: 4
- Rationale: Sweep — get_reg returns Err for names parse_reg_num cannot parse (x32, foo, empty). Matches llvm-mc rejection of unparsable registers.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_invalid_name
- Formal: ∀ mnemonic ∈ CRC32_8, slot ∈ {0,1,2}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_crc32(mnemonic, ops with slot=name) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, slot, name]
  domain: { mnemonic: eight CRC32 mnemonics, slot: 0..2, name: unparsable register names }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, ops_with_name_at(slot)).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  slot: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: oneof, items: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: get_reg / parse_reg_num None → Err; llvm-mc invalid operand
```

## encode_crc32_neg_nonreg
- Tier: 4
- Rationale: Sweep — get_reg requires Operand::Reg; Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement at any slot must Err.
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_neg_nonreg
- Formal: ∀ mnemonic ∈ CRC32_8, slot ∈ {0,1,2}, bad ∉ Reg. encode_crc32(mnemonic, ops with slot=bad) = Err
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, slot, bad]
  domain: { mnemonic: eight CRC32 mnemonics, slot: 0..2, bad: non-Reg Operand }
  relation:
    op: holds
    expr: encode_crc32(mnemonic, ops_with_nonreg_at(slot)).is_err()
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  slot: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: get_reg expected-register Err; llvm-mc invalid operand
```

## encode_crc32_diff_alt_spellings
- Tier: 5
- Rationale: Sweep — GNU-style aliases w31/WZR/uppercase W and x31/XZR/lr/uppercase X must agree with llvm-mc on the valid CRC32 domain (Rd/Rn always W; Rm W or X by form).
- Doc contract: bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd" — asserted fingerprint 61d3ed46
- Seed: encode_bfi_pbt encode_bfi_diff_alt_spellings
- Formal: ∀ mnemonic ∈ CRC32_8, rd,rn,rm ∈ 0..31, valid W/X aliases. encode_crc32(mnemonic, aliased_ops) = llvm-mc(aliased asm)
- Test file: src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_crc32
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm, dest_spell, src_n_spell, src_m_spell]
  domain: { mnemonic: eight CRC32 mnemonics, rd: 0..31, rn: 0..31, rm: 0..31, spellings: w31/WZR/uppercase/x31/XZR/lr }
  relation:
    op: eq
    lhs: encode_crc32(mnemonic, aliased_ops)
    rhs: llvm_mc(aliased_asm)
generators:
  mnemonic: { gen: oneof, items: ["crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 gas-compat; llvm-mc accepts w31/x31/WZR/XZR/lr aliases
```
