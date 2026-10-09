# Properties: encode_mov_reg_mem (i686)

## encode_mov_reg_mem_diff_llvm_mc_base_disp
- Tier: 4
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. State machine N/A (pure encoder). Round-trip rejected (no in-tree i686 MOV decoder). Reference KAT gate precedes PBT. Domain: same-width GP src + base+disp memory, no segment.
- Doc contract: (none) — function has no doc comment; Intel SDM MOV m,r / AT&T movl/movw/movb %reg,mem inferred. fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs base_disp differential (load twin)
- Formal: ∀ base ∈ GP32, disp ∈ i32, width ∈ {1,2,4}, src ∈ GP(width). bytes(encode_mov_reg_mem(src, mem(base,disp), width)) = llvm-mc(att_mov(width, src, mem))
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_reg_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp, width, src]
  domain: { base: GP32, disp: i32, width: "{1,2,4}", src: "GP(width)" }
  relation:
    op: eq
    lhs: "sut_encode(suffix(width), Register(src), Memory(base,disp))"
    rhs: "llvm_mc_bytes(att_asm)"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  src: { gen: oneof, values: ["al","ax","eax"], type: str }
evidence: gp_integer.rs:216-236; Intel SDM MOV 88/89; llvm-mc -triple=i686
```

## encode_mov_reg_mem_diff_llvm_mc_sib
- Tier: 4
- Rationale: SIB forms (ESP base, index≠esp, scales 1/2/4/8) densest ModR/M edge space; differential vs llvm-mc.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs sib differential
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp∈i32, width∈{1,2,4}, src∈GP(width). bytes(SUT mov reg→mem) = llvm-mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_reg_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp, width, src]
  domain: { index: "GP32\\{esp}", scale: "{1,2,4,8}", width: "{1,2,4}" }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(att_asm)"
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:216-236; encode_modrm_mem
```

## encode_mov_reg_mem_diff_llvm_mc_segment
- Tier: 4
- Rationale: All six segment overrides are valid on i686 (core.rs emit_segment_prefix). Body only accepts fs/gs — differential falsifies es/cs/ss/ds.
- Doc contract: (none) — body produces Err on non-fs/gs; not a domain restriction on the public MOV API. fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs segment differential
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base ∈ GP32, disp ∈ i32, width ∈ {1,2,4}, src ∈ GP(width). bytes(SUT) = llvm-mc(att with %seg:)
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0, width=1, src=al — asm `movb %al, %es:(%eax)`; SUT Err("unsupported segment: es"); llvm-mc [0x26, 0x88, 0x00]
- Bug report: bug_reports/encode_mov_reg_mem_missing_segment_prefix.md

```property
function: encode_mov_reg_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp, width, src]
  domain: { seg: "es|cs|ss|ds|fs|gs", width: "{1,2,4}" }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(att_asm)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: str }
evidence: core.rs:31-42 emit_segment_prefix; Intel SDM 2.1.1; gp_integer.rs:220-225
```

## encode_mov_reg_mem_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: ESP/EBP/SIB/absolute edges; skip llvm moffs A2/A3 encoding choice for eAX→abs.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs edges
- Formal: ∀ edge mem forms, width, src∈GP(width). if llvm not moffs then SUT bytes = llvm bytes
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_reg_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [edge, width, src]
  domain: { edge: "esp/ebp/sib/abs", width: "{1,2,4}" }
  relation:
    op: eq
    lhs: "sut"
    rhs: "llvm"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:216-236; Intel SDM MOV A2/A3 moffs optional
```

## encode_mov_reg_mem_invariant_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant — opcode 88/89, optional 0x66, ModRM.reg = src.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs invariant
- Formal: ∀ valid same-width GP store. strip_seg(bytes): if width=2 then 0x66; opcode=88 if width=1 else 89; ModRM.reg = gp_num(src)
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_reg_mem
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [base, disp, width, src]
  domain: { width: "{1,2,4}", src: "GP(width)" }
  relation:
    op: holds
    expr: "opcode_and_modrm_reg_match(bytes, width, src)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:226-235
```

## encode_mov_reg_mem_meta_store_vs_load_modrm
- Tier: 3
- Rationale: Metamorphic — same mem+GP → store (88/89) and load (8A/8B) share prefixes+ModRM/SIB/disp; only opcode differs. Required metamorphic/differential at standard tier.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs load_vs_store
- Formal: ∀ mem, reg, width. prefixes(store)=prefixes(load) ∧ tail(store)=tail(load) ∧ op(store)∈{88,89} ∧ op(load)∈{8A,8B}
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_reg_mem
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem, reg, width]
  domain: { width: "{1,2,4}" }
  relation:
    op: eq
    lhs: "modrm_tail(encode_mov_reg_mem(reg,mem,w))"
    rhs: "modrm_tail(encode_mov_mem_reg(mem,reg,w))"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:194-236 load/store pair
```

## encode_mov_reg_mem_neg_mismatched_width
- Tier: 4
- Rationale: Negative — mismatched GP width vs mnemonic must Err (llvm-mc rejects). Body has no reg_size gate.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs neg_mismatched_width
- Formal: ∀ mnemonic width W, src with reg_size≠W. llvm-mc rejects ⇒ SUT returns Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: failing
- Counterexample: mnemonic=movl, src=ax, mem=(%eax) → Ok([0x89, 0x00]); llvm-mc rejects
- Bug report: bug_reports/encode_mov_reg_mem_mismatched_width.md

```property
function: encode_mov_reg_mem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, src, mem]
  domain: { "reg_size(src) != mnemonic_width" }
  relation:
    op: throws
    expr: "sut_encode(mnemonic, Register(src), Memory(mem))"
expected_error: String
generators:
  mnemonic: { gen: oneof, values: ["movb","movw","movl"], type: str }
evidence: Intel SDM MOV size match; llvm-mc rejects; gp_integer.rs:217 no size gate
```

## encode_mov_reg_mem_neg_non_gp_src
- Tier: 4
- Rationale: Negative — xmm/mm/st/ymm src must Err; reg_num aliases them to 0-7.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs neg_non_gp_dest
- Formal: ∀ non_gp ∈ {xmm*,mm*,st*,ymm*}, width. llvm-mc rejects ⇒ SUT returns Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs
- Status: failing
- Counterexample: non_gp=xmm0, width=1, mem=(%eax) — `movb %xmm0, (%eax)` → Ok([0x88, 0x00]); llvm-mc rejects
- Bug report: bug_reports/encode_mov_reg_mem_non_gp_src.md

```property
function: encode_mov_reg_mem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [non_gp, width, mem]
  domain: { non_gp: "xmm|mm|st|ymm" }
  relation:
    op: throws
    expr: "sut_encode(suffix(width), Register(non_gp), Memory(mem))"
expected_error: String
generators:
  non_gp: { gen: oneof, values: ["xmm0","mm0","st","ymm0"], type: str }
evidence: Intel SDM MOV r/m,r is GP; registers.rs:4-14 aliases
```
