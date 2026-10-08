# Properties: encode_prefetch

## encode_prefetch_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler (Intel PREFETCHh 0F 18 /hint). State machine N/A. Round-trip N/A (no decoder). SUT-boundary=internal-helper of GNU-style i686 assembler.
- Doc contract: system.rs:10 "Encode prefetch instructions (0F 18 /hint)" — asserted fingerprint 7a3c91e2
- Seed: (none)
- Formal: ∀ m ∈ {prefetcht0,prefetcht1,prefetcht2,prefetchnta}, ∀ mem ∈ ValidMem32 (no segment). encode(m, [Mem(mem)]).bytes = llvm-mc(-triple=i686, m mem)
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, mem]
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  base: { gen: oneof, values: ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp"] }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:10 + encoder/mod.rs:739-742 + Intel SDM PREFETCHh
```

## encode_prefetch_hint_modrm_reg
- Tier: 4
- Rationale: Algebraic invariant — opcode is always 0F 18 and ModRM.reg equals dispatched hint.
- Doc contract: system.rs:10 "Encode prefetch instructions (0F 18 /hint)" — asserted fingerprint 7a3c91e2
- Seed: (none)
- Formal: ∀ m∈Mnemonics, ∀ mem∈ValidMem32. let b=encode(m,[Mem(mem)]).bytes in b[0]=0x0F ∧ b[1]=0x18 ∧ ((b[2]>>3)&7)=hint(m)
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, mem]
  relation:
    op: holds
    expr: bytes_ok_opcode_and_hint
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  base: { gen: oneof, values: ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp"] }
  disp: { gen: int, min: -200, max: 300, type: i64 }
evidence: system.rs:10-18; mod.rs:739-742
```

## encode_prefetch_meta_hint_isolates_reg
- Tier: 4
- Rationale: Metamorphic — changing only mnemonic/hint changes only ModRM.reg.
- Doc contract: system.rs:10 "Encode prefetch instructions (0F 18 /hint)" — asserted fingerprint 7a3c91e2
- Seed: (none)
- Formal: ∀ m1,m2∈Mnemonics, ∀ mem. let a=encode(m1,[mem]), b=encode(m2,[mem]) in a[0..2)=b[0..2)=[0F,18] ∧ a[3..]=b[3..] ∧ ((a[2]>>3)&7)=hint(m1) ∧ ((b[2]>>3)&7)=hint(m2) ∧ (a[2]&0xC7)=(b[2]&0xC7)
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [m1, m2, mem]
  relation:
    op: holds
    expr: hint_isolates_modrm_reg
generators:
  m1: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  m2: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  base: { gen: oneof, values: ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp"] }
  disp: { gen: int, min: -1000, max: 1000, type: i64 }
evidence: system.rs:16-18
```

## encode_prefetch_neg_arity
- Tier: 3
- Rationale: Negative/error — arity must be exactly 1.
- Doc contract: system.rs:12-14 arity guard — asserted fingerprint b2e81c04
- Seed: (none)
- Formal: ∀ m∈Mnemonics, ∀ ops. len(ops)≠1 ⇒ encode(m,ops)=Err
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, ops]
  relation:
    op: throws
    expr: sut_encode_wrong_arity
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  arity: { gen: int, min: 0, max: 5, type: usize }
expected_error: prefetch requires 1 operand
evidence: system.rs:12-14
```

## encode_prefetch_neg_non_memory
- Tier: 3
- Rationale: Negative/error — non-memory operands rejected.
- Doc contract: system.rs:20 "prefetch requires memory operand" — asserted fingerprint 9d4a12f0
- Seed: (none)
- Formal: ∀ m∈Mnemonics, ∀ op∉Memory. encode(m,[op])=Err
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, op]
  relation:
    op: throws
    expr: sut_encode_non_memory
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  kind: { gen: int, min: 0, max: 3, type: u8 }
expected_error: prefetch requires memory operand
evidence: system.rs:20
```

## encode_prefetch_diff_sib_esp_ebp_edges
- Tier: 5
- Rationale: Differential on ESP/EBP/scale/disp boundary edges vs llvm-mc.
- Doc contract: system.rs:10 plus encode_modrm_mem ESP/EBP rules in core.rs:84-97
- Seed: (none)
- Formal: ∀ m, ∀ edge_mem ∈ EspEbpScaleDispEdges. encode(m,[edge_mem])=llvm-mc(m, att(edge_mem))
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, edge_mem]
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  edge: { gen: int, min: 0, max: 11, type: u8 }
evidence: core.rs:84-97; Intel Vol.2 ModR/M
```

## encode_prefetch_diff_segment_prefix
- Tier: 5
- Rationale: Differential — segment overrides must emit prefix before 0F 18, matching llvm-mc and sibling encode_sse_mem_only.
- Doc contract: system.rs:10; sibling x86/encoder/sse.rs:611; i686 core.rs:31-42
- Seed: (none)
- Formal: ∀ m, ∀ seg∈{fs,gs,es,cs,ss,ds}, ∀ base. encode(m,[Mem(seg:base)]) = llvm-mc(m, %seg:(%base))
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: failing
- Counterexample: mnemonic=prefetcht0, seg=es, base=eax, disp=0 → SUT=[0f,18,08] llvm-mc=[26,0f,18,08]
- Bug report: bug_reports/encode_prefetch_missing_segment_prefix.md

```property
function: i686.encoder.encode_prefetch
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, seg, base, disp]
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  seg: { gen: oneof, values: ["es", "cs", "ss", "ds", "fs", "gs"] }
  base: { gen: oneof, values: ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp"] }
  disp: { gen: int, min: -4, max: 127, type: i64 }
evidence: core.rs:31-42; x86 sse.rs:611; llvm-mc i686
```

## encode_prefetch_opcode_len_ge3
- Tier: 3
- Rationale: Algebraic invariant — successful encode yields at least 3 bytes starting with 0F 18.
- Doc contract: system.rs:16-18
- Seed: (none)
- Formal: ∀ m, ∀ valid mem. |encode(m,[mem]).bytes| ≥ 3 ∧ starts with 0F 18
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, mem]
  relation:
    op: holds
    expr: bytes_len_ge3_and_opcode_0f18
generators:
  mnemonic: { gen: oneof, values: ["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"] }
  base: { gen: oneof, values: ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp"] }
  disp: { gen: int, min: -65536, max: 65536, type: i64 }
evidence: system.rs:16-18
```
