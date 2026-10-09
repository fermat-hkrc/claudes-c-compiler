# Properties: encode_bit_count (i686)

## encode_bit_count_diff_rr_llvm_mc
- Tier: 4
- Rationale: Strongest runnable oracle is differential vs llvm-mc i686. State machine N/A (pure encoder). Round-trip N/A (no in-tree i686 decoder). Intel SDM Vol.2 LZCNT/TZCNT/POPCNT r32,r/m32 with F3 0F BD/BC/B8 /r; AT&T src,dst order.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: (none); pattern from encode_bswap_pbt.rs differential
- Formal: ∀ m ∈ {lzcntl,tzcntl,popcntl}, ∀ s,d ∈ GP32. encode(m, Reg(s), Reg(d)) = llvm-mc("m %s, %d")
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.gp_integer.encode_bit_count
oracle: differential
predicate:
  quantifier: forall
  vars: [m, s, d]
  domain: { m: {lzcntl,tzcntl,popcntl}, s: GP32, d: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(s), Reg(d)])"
    rhs: "llvm_mc_bytes(f\"{m} %{s}, %{d}\")"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:959-981; mod.rs:267; Intel SDM LZCNT/TZCNT/POPCNT; llvm-mc i686
```

## encode_bit_count_invariant_opcode
- Tier: 3
- Rationale: Algebraic invariant from Intel opcode map — F3 ‖ opc ‖ ModRM(mod=3,reg=dst,rm=src). Weaker than differential but pins the structural layout without an external tool.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: (none)
- Formal: ∀ m ∈ {lzcntl,tzcntl,popcntl}, ∀ s,d ∈ GP32. encode(m,Reg(s),Reg(d)) = [0xF3, 0x0F, opc(m), ModRM(3,d,s)] where opc(lzcntl)=0xBD, opc(tzcntl)=0xBC, opc(popcntl)=0xB8
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.gp_integer.encode_bit_count
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [m, s, d]
  domain: { m: bitcount_mnems, s: GP32, d: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(m,[Reg(s),Reg(d)])"
    rhs: "[0xF3, 0x0F, opc(m), modrm(3, reg_num(d), reg_num(s))]"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:964-977; Intel SDM opcode map
```

## encode_bit_count_meta_modrm_swap
- Tier: 3
- Rationale: Metamorphic — swapping src/dst must change ModRM when s≠d (reg and r/m fields swap roles); re-encode is deterministic.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: (none)
- Formal: ∀ m, ∀ s≠d ∈ GP32. encode(m,s,d) ≠ encode(m,d,s) ∧ encode(m,s,d) is deterministic
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.gp_integer.encode_bit_count
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [m, s, d]
  domain: { m: bitcount_mnems, s: GP32, d: GP32, s != d }
  relation:
    op: ne
    lhs: "sut_encode(m,[Reg(s),Reg(d)])"
    rhs: "sut_encode(m,[Reg(d),Reg(s)])"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:977 modrm(3, dst_num, src_num)
```

## encode_bit_count_diff_mem_llvm_mc
- Tier: 4
- Rationale: Intel SDM and llvm-mc accept memory source (r32, r/m32). SUT match arm only handles Reg/Reg and rejects Mem — gap vs SDM/x86 sibling encode_bit_count which encodes Mem.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: (none); x86 sibling gp_integer.rs:912-920 supports Memory
- Formal: ∀ m ∈ {lzcntl,tzcntl,popcntl}, ∀ base ∈ GP32, ∀ d ∈ GP32. encode(m, Mem(base), Reg(d)) = llvm-mc("m (base), %d")
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: failing
- Counterexample: lzcntl (%eax), %eax → Err("unsupported lzcntl operands"); llvm-mc=[f3,0f,bd,00]
- Bug report: bug_reports/encode_bit_count_mem_src.md

```property
function: encoder.gp_integer.encode_bit_count
oracle: differential
predicate:
  quantifier: forall
  vars: [m, base, d]
  domain: { m: bitcount_mnems, base: GP32, d: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(m,[Mem(base),Reg(d)])"
    rhs: "llvm_mc_bytes(f\"{m} (%{base}), %{d}\")"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: Intel SDM LZCNT r32,r/m32; llvm-mc accepts; x86 encode_bit_count memory arm
```

## encode_bit_count_neg_arity
- Tier: 3
- Rationale: Negative/error contract — ops.len() must be 2 (gp_integer.rs:960-962).
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: encode_bswap_pbt encode_bswap_neg_arity
- Formal: ∀ m, ∀ n ≠ 2, ∀ ops with |ops|=n. encode(m, ops) = Err ∧ message contains "requires 2 operands"
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.gp_integer.encode_bit_count
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, n]
  domain: { m: bitcount_mnems, n: 0..5, n != 2 }
  relation:
    op: holds
    expr: "sut_encode(m, ops_n).is_err()"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  n: { gen: int, min: 0, max: 4, type: usize }
expected_error: "requires 2 operands"
evidence: gp_integer.rs:960-962
```

## encode_bit_count_neg_wrong_width
- Tier: 4
- Rationale: lzcntl/tzcntl/popcntl are 32-bit forms; llvm-mc rejects r16/r8 mixed with *l. SUT uses reg_num which aliases ax/al onto the same 3-bit codes — must still reject wrong width.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: encode_bswap_pbt encode_bswap_neg_wrong_width
- Formal: ∀ m, ∀ s ∈ BAD_WIDTH ∪ GP32, ∀ d ∈ BAD_WIDTH ∪ GP32. (s ∉ GP32 ∨ d ∉ GP32) ⇒ encode(m,Reg(s),Reg(d)) = Err ∧ llvm-mc rejects
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: failing
- Counterexample: lzcntl %ax, %eax → Ok([f3,0f,bd,c0])
- Bug report: bug_reports/encode_bit_count_wrong_width.md

```property
function: encoder.gp_integer.encode_bit_count
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, s, d]
  domain: { m: bitcount_mnems, at least one of s,d in r16/r8 }
  relation:
    op: holds
    expr: "sut_encode(m,[Reg(s),Reg(d)]).is_err()"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  s: { gen: oneof, values: ["ax","cx","al","cl","eax","ecx"] }
  d: { gen: oneof, values: ["bx","dx","bl","dl","ebx","edx"] }
expected_error: invalid operand width
evidence: Intel SDM; llvm-mc rejects lzcntl %ax,%ebx; registers.rs:4-14 aliases
```

## encode_bit_count_neg_non_gp
- Tier: 4
- Rationale: Non-GP names (xmm/mm/st/ymm) that reg_num aliases must be rejected; llvm-mc rejects them.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: encode_bswap_pbt encode_bswap_neg_non_gp
- Formal: ∀ m, ∀ bad ∈ NON_GP_ALIASED, ∀ d ∈ GP32. encode(m, Reg(bad), Reg(d)) = Err ∧ encode(m, Reg(d), Reg(bad)) = Err
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: failing
- Counterexample: lzcntl %xmm0, %eax → Ok([f3,0f,bd,c0])
- Bug report: bug_reports/encode_bit_count_non_gp.md

```property
function: encoder.gp_integer.encode_bit_count
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, bad, d]
  domain: { m: bitcount_mnems, bad: NON_GP_ALIASED, d: GP32 }
  relation:
    op: holds
    expr: "sut_encode(m,[Reg(bad),Reg(d)]).is_err() && sut_encode(m,[Reg(d),Reg(bad)]).is_err()"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  bad: { gen: oneof, values: ["xmm0","mm0","st(0)","ymm0"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
expected_error: bad/non-GP register
evidence: registers.rs:4-14; llvm-mc rejects
```

## encode_bit_count_neg_imm_label
- Tier: 3
- Rationale: Negative/error contract for inputs Intel SDM declares invalid for LZCNT/TZCNT/POPCNT (forms are only r32,r/m32 — Immediate, Label, and memory-as-destination are out of domain). SUT must reject them with Err.
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: encode_bswap_pbt encode_bswap_neg_non_register
- Formal: ∀ m, ∀ kind ∈ {Imm, Label, Mem-as-dst}. encode(m, kind) = Err (Intel SDM: only r32, r/m32 forms exist)
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.gp_integer.encode_bit_count
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, kind]
  domain: { m: bitcount_mnems, kind: invalid operand pairs }
  relation:
    op: holds
    expr: "sut_encode(m, invalid_ops(kind)).is_err()"
generators:
  m: { gen: oneof, values: ["lzcntl", "tzcntl", "popcntl"] }
  kind: { gen: int, min: 0, max: 3, type: u8 }
expected_error: unsupported operands
evidence: gp_integer.rs:980; Intel SDM only r32,r/m32 forms
```

## encode_bit_count_meta_mnemonic_opcodes_differ
- Tier: 3
- Rationale: Strengthen round — metamorphic: lzcntl/tzcntl/popcntl must emit distinct opcodes 0xBD/0xBC/0xB8 for identical register pairs (Intel opcode map).
- Doc contract: gp_integer.rs:959 (none — no doc comment on encode_bit_count) — other fingerprint e120fe10
- Seed: (none)
- Formal: ∀ s,d ∈ GP32. encode(lzcntl,s,d)[2]=0xBD ∧ encode(tzcntl,s,d)[2]=0xBC ∧ encode(popcntl,s,d)[2]=0xB8 ∧ pairwise distinct
- Test file: src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.gp_integer.encode_bit_count
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [s, d]
  domain: { s: GP32, d: GP32 }
  relation:
    op: holds
    expr: "opc_byte(lzcntl,s,d)==0xBD && opc_byte(tzcntl,s,d)==0xBC && opc_byte(popcntl,s,d)==0xB8 && distinct"
generators:
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:964-968
```
