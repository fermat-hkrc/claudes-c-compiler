# Properties: encode_bt (i686)

## encode_bt_diff_rr
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine N/A; no in-tree decoder for round-trip. Shared AT&T BT r32,r32 contract.
- Doc contract: (none) — encode_bt has no doc comment
- Seed: (none)
- Formal: ∀ mnem ∈ {btl,btsl,btrl,btcl,bt,bts,btr,btc}, ∀ src,dst ∈ GP32. encode_bt([%src,%dst], mnem) = llvm-mc(mnem %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, src, dst]
  domain: { mnem: bt_family, src: gp32, dst: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Reg(src), Reg(dst)])"
    rhs: "llvm_mc(mnem %src, %dst)"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl","bt","bts","btr","btc"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:1012 + Intel SDM BT 0F A3/AB/B3/BB + llvm-mc i686
```

## encode_bt_diff_imm_reg
- Tier: 4
- Rationale: Imm8 bit-offset form 0F BA /ext is the other primary encoding path; differential vs llvm-mc over imm ∈ [-128,255].
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem ∈ MNEMS_L, ∀ dst ∈ GP32, ∀ imm ∈ imm8_domain. encode_bt([$imm,%dst], mnem) = llvm-mc(mnem $imm, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, dst, imm]
  domain: { mnem: bt_l_family, dst: gp32, imm: imm8 }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Imm(imm), Reg(dst)])"
    rhs: "llvm_mc(mnem $imm, %dst)"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  imm: { gen: int, min: -128, max: 255, type: i64 }
evidence: gp_integer.rs:1038-1044 + Intel SDM 0F BA /4..7 ib
```

## encode_bt_diff_reg_mem_bare
- Tier: 4
- Rationale: Reg→Mem form is architecturally valid (BT r/m, r); bare memory without segment should match llvm-mc.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem ∈ MNEMS_L, ∀ src ∈ GP32, ∀ mem ∈ bare_mem32. encode_bt([%src,mem], mnem) = llvm-mc(mnem %src, mem)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, src, mem]
  domain: { mnem: bt_l_family, src: gp32, mem: bare_i686_mem }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Reg(src), Mem(mem)])"
    rhs: "llvm_mc(mnem %src, att(mem))"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  mem: { gen: bare_mem }
evidence: gp_integer.rs:1033-1037 + Intel SDM BT r/m32,r32
```

## encode_bt_diff_imm_mem_bare
- Tier: 4
- Rationale: Imm→Mem form 0F BA /ext + modrm_mem + imm8 is a distinct code path including SIB/abs.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem ∈ MNEMS_L, ∀ imm ∈ imm8, ∀ mem ∈ bare_mem32. encode_bt([$imm,mem], mnem) = llvm-mc(mnem $imm, mem)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, imm, mem]
  domain: { mnem: bt_l_family, imm: imm8, mem: bare_i686_mem }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Imm(imm), Mem(mem)])"
    rhs: "llvm_mc(mnem $imm, att(mem))"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl"] }
  imm: { gen: int, min: 0, max: 255, type: i64 }
  mem: { gen: bare_mem }
evidence: gp_integer.rs:1045-1050
```

## encode_bt_diff_segment
- Tier: 4
- Rationale: core.rs emit_segment_prefix documents all six overrides; x86-64 sibling encode_bt calls emit_segment_prefix on mem arms; llvm-mc emits prefix. Differential must hold for %es:/ %fs: etc.
- Doc contract: (none) — function has no comment; contract inferred from core.rs emit_segment_prefix + x86-64 sibling
- Seed: encode_test_pbt encode_test_diff_segment_prefix
- Formal: ∀ mnem, ∀ form ∈ {reg_mem,imm_mem}, ∀ seg ∈ SREG, ∀ bare_mem. encode_bt(..., seg:mem) = llvm-mc(... %seg:mem)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: failing
- Counterexample: btl %eax, %es:(%eax) -> sut=[0f,a3,00] mc=[26,0f,a3,00]
- Bug report: bug_reports/encode_bt_missing_segment_prefix.md

```property
function: encode_bt
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, form, seg, mem]
  domain: { mnem: bt_l_family, form: {reg_mem,imm_mem}, seg: sreg, mem: base_mem }
  relation:
    op: eq
    lhs: "sut_encode(mnem, segmented ops)"
    rhs: "llvm_mc(AT&T with %seg:)"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl"] }
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: core.rs:31-42 emit_segment_prefix; x86 x87_misc.rs encode_bt emit_segment_prefix on mem
```

## encode_bt_meta_bare_equiv_l
- Tier: 3
- Rationale: Algebraic metamorphic — dispatch aliases bare mnemonics to same encode_bt path; bare must equal *l encoding for r32.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ base ∈ {bt,bts,btr,btc}, ∀ src,dst ∈ GP32. encode(base) = encode(base+"l")
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [base, src, dst]
  domain: { base: {bt,bts,btr,btc}, src: gp32, dst: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(base, [src,dst])"
    rhs: "sut_encode(base+l, [src,dst])"
generators:
  base: { gen: oneof, values: ["bt","bts","btr","btc"] }
evidence: mod.rs:274 bare and *l share encode_bt
```

## encode_bt_meta_segment_prefix
- Tier: 3
- Rationale: Algebraic metamorphic — segmented form must equal seg_prefix_byte ‖ bare encoding (core.rs contract). Independent of llvm-mc; same defective arms as differential segment.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem, ∀ seg, ∀ mem. encode(seg:mem) = [seg_prefix(seg)] ‖ encode(bare mem)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: failing
- Counterexample: btl %es:(%eax) -> got [0f,a3,00] expect [26,0f,a3,00]
- Bug report: bug_reports/encode_bt_meta_segment_prefix.md

```property
function: encode_bt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mnem, seg, mem]
  domain: { mnem: bt_l_family, seg: sreg, mem: base_mem }
  relation:
    op: eq
    lhs: "sut_encode(seg:mem)"
    rhs: "[seg_prefix] ++ sut_encode(bare)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: core.rs:31-42 emit_segment_prefix
```

## encode_bt_invariant_rr_layout
- Tier: 3
- Rationale: Algebraic invariant from Intel SDM — bytes are exactly [0F, opc, ModRM(3,src,dst)].
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem ∈ MNEMS_L, ∀ src,dst ∈ GP32. bytes = [0F, opc(mnem), modrm(3,num(src),num(dst))]
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnem, src, dst]
  domain: { mnem: bt_l_family, src: gp32, dst: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(mnem, RR)"
    rhs: "[0x0F, opc_rr(mnem), modrm(3,src,dst)]"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl"] }
evidence: gp_integer.rs:1026-1032; Intel SDM
```

## encode_bt_diff_label
- Tier: 4
- Rationale: Label absolute forms (Imm→Label, Reg→Label) are explicit arms in encode_bt; differential vs llvm-mc with fixup bytes as zero.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem, ∀ form ∈ {imm_label,reg_label}. encode_bt(label form) = llvm-mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, form, label]
  domain: { mnem: bt_l_family, form: {imm,reg}, label: symbol }
  relation:
    op: eq
    lhs: "sut_encode(label form)"
    rhs: "llvm_mc(label form)"
generators:
  mnem: { gen: oneof, values: ["btl","btsl","btrl","btcl"] }
evidence: gp_integer.rs:1051-1070
```

## encode_bt_neg_arity
- Tier: 3
- Rationale: Negative — arity≠2 must Err (documented by encode_bt guard).
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem, ∀ arity ≠ 2. encode_bt(ops_arity).is_err()
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, arity]
  domain: { mnem: bt_l_family, arity: {0,1,3} }
  relation:
    op: holds
    expr: "sut_encode(mnem, ops_of_arity).is_err()"
generators:
  arity: { gen: int, min: 0, max: 3 }
expected_error: String
evidence: gp_integer.rs:1013-1014
```

## encode_bt_neg_wrong_width
- Tier: 3
- Rationale: Negative — r16/r8 with *l forms that llvm-mc rejects must Err (not reg_num alias).
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem ∈ MNEMS_L, ∀ (src,dst) with a non-r32 GP name that llvm-mc rejects. encode_bt.is_err()
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: failing
- Counterexample: btl %ax, %eax -> Ok([0f,a3,c0])
- Bug report: bug_reports/encode_bt_wrong_width_alias.md

```property
function: encode_bt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, src, dst]
  domain: { mnem: bt_l_family, src_or_dst: r16_or_r8 }
  relation:
    op: holds
    expr: "sut_encode(mnem, [Reg(src), Reg(dst)]).is_err()"
generators:
  src: { gen: oneof, values: ["ax","al","eax"] }
expected_error: String
evidence: Intel BT *l is r32; llvm-mc rejects btl %ax, %ebx
```

## encode_bt_neg_imm_oor
- Tier: 3
- Rationale: Negative — imm outside imm8 domain llvm-mc rejects must Err (not `as u8` truncate).
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem, ∀ imm ∉ [-128,255] that llvm-mc rejects. encode_bt([$imm,%dst]).is_err()
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: failing
- Counterexample: btl $256, %eax -> Ok([0f,ba,e0,00])
- Bug report: bug_reports/encode_bt_imm_oor_truncates.md

```property
function: encode_bt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, imm, dst]
  domain: { mnem: bt_l_family, imm: oor_imm, dst: gp32 }
  relation:
    op: holds
    expr: "sut_encode(mnem, [Imm(imm), Reg(dst)]).is_err()"
generators:
  imm: { gen: int, min: 256, max: 1000, type: i64 }
expected_error: String
evidence: Intel 0F BA ib; llvm-mc rejects $256; gp_integer.rs:1041 `*val as u8`
```

## encode_bt_invariant_imm_reg_layout
- Tier: 3
- Rationale: Strengthen round — Imm→Reg layout [0F, BA, ModRM(3,ext,dst), imm8] structural invariant.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ mnem ∈ MNEMS_L, ∀ dst ∈ GP32, ∀ imm ∈ 0..255. bytes = [0F, BA, modrm(3,ext,dst), imm]
- Test file: src/backend/i686/assembler/encoder/encode_bt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bt
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnem, dst, imm]
  domain: { mnem: bt_l_family, dst: gp32, imm: u8 }
  relation:
    op: eq
    lhs: "sut_encode(imm→reg)"
    rhs: "[0x0F, 0xBA, modrm(3,ext,dst), imm]"
generators:
  imm: { gen: int, min: 0, max: 255, type: u8 }
evidence: gp_integer.rs:1038-1044; Intel SDM
```
