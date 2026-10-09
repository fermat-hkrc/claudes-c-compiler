# Confirmed invariants / environment quirks

## encode_in / encode_out (i686 system encoder)

- Intel IN/OUT fix data register to AL/AX/EAX and port to DX (or imm8).
- AT&T operand order: OUT is `outb %al, %dx` / `outb %al, $imm`; IN is `inb %dx, %al` / `inb $imm, %al`.
- AT&T parenthesized port `(%dx)` is accepted by llvm-mc as alias of `%dx` (same EC/ED or EE/EF bytes).
- Operand-size override: inw/outw = `[0x66] ++ inl/outl` for the same port shape.
- llvm-mc path used by campaigns: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases set explicitly to 1000 for standard tier.
- Known defect class (both encode_in and encode_out): (1) any Register,Register pair accepted; (2) `*val as u8` truncates OOR imm; (3) missing Memory port form on i686 (x86-64 sibling handles it).

## encode_invlpg (i686 system encoder)

- Opcode: 0F 01 /7 + ModR/M memory only (rejects register/imm/label).
- Non-segment base/disp/SIB/abs forms match llvm-mc `-triple=i686`.
- Metamorphic: same memory → invlpg Mod+RM/SIB/disp equals lidt; only ModRM.reg differs (7 vs 3).
- Known defect: does not call `emit_segment_prefix` before opcode (same class as encode_prefetch / encode_prefetch_0f0d). Witness: `invlpg %es:(%eax)` → SUT omits 0x26.
- Fix shape: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`.

## encode_verw (i686 system encoder)

- Opcode: 0F 00 /5 + ModR/M; Intel VERW is r/m16 (register form: ax/bx/cx/dx/sp/bp/si/di only).
- Non-segment base/disp/SIB/abs and r16 forms match llvm-mc `-triple=i686`.
- Known defects: (1) memory arm does not call `emit_segment_prefix` before opcode (same class as encode_invlpg/prefetch); (2) register arm accepts 32/8-bit names via `reg_num` aliasing (eax→same bytes as ax).
- Fix shapes: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x00])`; gate register form with `reg_size(&reg.name) == 2`.

## Harness

- PBT files: `src/backend/i686/assembler/encoder/*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`.
- Build contract form: `cargo test --lib <filter> -- --test-threads=1`.
- llvm-mc path: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases set explicitly to 1000 for standard tier.

## encode_lsl (i686 system encoder)

- Opcode: 0F 03 /r (Load Segment Limit); AT&T `lsl src, dst` with dest in ModRM.reg.
- Same-width reg32/reg16 forms match llvm-mc `-triple=i686`.
- Operand-size override MUST follow destination width (Intel LSL r16/r32, r/m16), not source.
- Memory form must: (1) `emit_segment_prefix(mem)` before opcode; (2) emit 0x66 when dest is r16.
- Known defects: (1) memory arm skips segment prefix; (2) register arm keys is_16 off src; (3) memory arm never emits 0x66 for r16 dest.
- Fix shape: `is_16` on `dst.name`; memory arm `emit_segment_prefix` + optional 0x66 then `0F 03` + modrm_mem.

## encode_system_table (i686 system encoder)

- Opcode: 0F 01 /N with N={sgdt:0,sidt:1,lgdt:2,lidt:3}; optional `l` suffix stripped (`lgdtl`→`lgdt`).
- Memory forms (base/disp/SIB/abs, no segment) and Label absolute form match llvm-mc `-triple=i686`.
- Metamorphic: same memory → four mnemonics share mod+rm/SIB/disp; only ModRM.reg differs.
- Label form: mod=00 rm=101 + R_386_32 reloc + 4 zero bytes (llvm-mc `A,A,A,A`).
- Known defect: memory arm does not call `emit_segment_prefix` before opcode (same class as encode_invlpg/prefetch/verw/lsl). Witness: `sgdt %es:(%eax)` → SUT omits 0x26; `lgdt %fs:(%eax)` omits 0x64.
- Fix shape: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`.

## encode_lmsw (i686 system encoder)

- Opcode: 0F 01 /6 + ModR/M; Intel LMSW is r/m16 (register form: ax/bx/cx/dx/sp/bp/si/di only). No 0x66 prefix on register form (unlike SMSW).
- Non-segment base/disp/SIB/abs and r16 forms match llvm-mc `-triple=i686`.
- Metamorphic: same memory → lmsw Mod+RM/SIB/disp equals lidt; only ModRM.reg differs (6 vs 3).
- Known defects: (1) memory arm does not call `emit_segment_prefix` before opcode (same class as encode_invlpg/prefetch/verw/lsl/system_table). Witness: `lmsw %es:(%eax)` → SUT omits 0x26; `lmsw %fs:4(%esi)` omits 0x64. (2) register arm accepts 32/8-bit names via `reg_num` aliasing (eax→same bytes as ax).
- Fix shapes: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`; gate register form with `reg_size(&reg.name) == 2`.

## encode_smsw (i686 system encoder)

- Opcode: 0F 01 /4 + ModR/M; Intel SMSW is r/m16 or r32/m16 (register form: r16 gets 0x66 operand-size prefix; r32 does not).
- Non-segment base/disp/SIB/abs and r16/r32 forms match llvm-mc `-triple=i686`.
- Metamorphic: same memory → smsw Mod+RM/SIB/disp equals lidt; only ModRM.reg differs (4 vs 3).
- Known defects: (1) memory arm does not call `emit_segment_prefix` before opcode (same class as encode_lmsw/invlpg/prefetch/verw/lsl/system_table). Witness: `smsw %es:(%eax)` → SUT omits 0x26; `smsw %fs:4(%esi)` omits 0x64. (2) register arm accepts 8-bit names via `reg_num` aliasing (al→same bytes as eax, no 0x66).
- Fix shapes: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`; gate register form with `reg_size ∈ {2,4}` (keep 0x66 for size 2).
- Note: unlike LMSW, SMSW legitimately accepts r32 (smswl); do not reject eax.

## encode_mov_cr (i686 system encoder)

- Opcode: 0F 20 /r (CR→GP read) or 0F 22 /r (GP→CR write); ModRM mod=3, reg=CR number, rm=GP number.
- Control regs recognized: cr0, cr2, cr3, cr4 (`is_control_reg` / `control_reg_num`); cr1 not in the set (dispatch never reaches encode_mov_cr for cr1).
- Valid GP forms are r32 only (eax..edi). AT&T `movl %cr0, %eax` / `movl %eax, %cr0`.
- Non-segment r32×CR forms match llvm-mc `-triple=i686`.
- Metamorphic: same CR/GP → read and write share ModRM; only opcode byte differs (0x20 vs 0x22).
- Known defect: register arm accepts 8/16-bit GP names via `reg_num` aliasing (ax/al → same bytes as eax); also accepts `movw` CR forms. Witness: `movl %cr0, %ax` → `[0f,20,c0]`; llvm-mc rejects.
- Fix shape: `if reg_size(&gp.name) != 4 { return Err(...); }` on both arms before `reg_num`.

## encode_mov_rr (i686 gp_integer encoder)

- GP RR path: opcodes 0x88 (size=1) / 0x89 (else) + optional leading 0x66 when size=2; ModRM mod=3, reg=src, rm=dst (AT&T %src, %dst).
- Same-width GP pairs (r8/r16/r32 including ah..bh) match llvm-mc `-triple=i686`.
- Metamorphic: swap src/dst swaps ModRM.reg ↔ ModRM.rm; identity mov is well-formed.
- Segment arms inside encode_mov_rr (8E/8C) are normally bypassed: encode_mov routes segment regs to encode_mov_seg first.
- Known defects: (1) no width check — mismatched GP names accepted via reg_num aliasing (movl %ax,%ebx → same as movl %eax,%ebx); (2) non-GP names (xmm/mm/st) accepted via reg_num (movl %xmm0,%eax → movl %eax,%eax bytes).
- Fix shape: before reg_num on GP path, require `reg_size(src)==size && reg_size(dst)==size` and reject xmm/mm/st/ymm prefixes.

## encode_mov_seg (i686 system encoder)

- Opcode: 8C /r (Sreg→r/m) or 8E /r (r/m→Sreg); ModRM.reg = segment number (es=0..gs=5).
- Segment regs: es, cs, ss, ds, fs, gs (`is_segment_reg` / local seg_num).
- r32 forms (`movl %ds, %eax` / `movl %eax, %ds`) match llvm-mc `-triple=i686` with no 0x66.
- r16 → Sreg (`movw %ax, %ds`) matches llvm-mc with no 0x66.
- Memory forms without segment override (base/disp/SIB/abs) match llvm-mc under `movw`.
- Metamorphic: same sreg/GP → read and write share ModRM; only opcode differs (0x8C vs 0x8E).
- Known defects: (1) memory arms do not call `emit_segment_prefix` before opcode (same class as lmsw/invlpg/smsw/…). Witness: `movw %ds, %es:(%eax)` → SUT omits 0x26. (2) Sreg→r16 omits 0x66 (`movw %ds, %ax` → `[8c,d8]` vs `[66,8c,d8]`). (3) register arms accept r8 via `reg_num` aliasing (`movl %al, %ds` → same as eax).
- Fix shapes: `self.emit_segment_prefix(mem);` before 0x8C/0x8E on mem arms; `if reg_size(&gp)==2 { push 0x66 }` on Sreg→GP arm; `if reg_size(&gp)==1 { return Err }` on both register arms.

## encode_pop16 (i686 system encoder)

- Opcode forms: r16 short `66 58+rw`; Sreg one-byte/0F (`07`/`17`/`1F`/`0F A1`/`0F A9`) **must** carry `0x66` under `popw` (llvm-mc); memory `66 8F /0` (+ optional segment prefix).
- GP r16 forms match llvm-mc `-triple=i686`. Metamorphic: popw r16 = `[0x66] ‖` popl r32 short form.
- Known defects: (1) Sreg arm omits 0x66 (comment at system.rs:333 is wrong for popw; true for popl). Witness: `popw %es` → `[07]` vs `[66,07]`. (2) Memory arm missing entirely (`unsupported popw operand`). Witness: `popw (%eax)`. (3) non-segment arm accepts r32/r8 via `reg_num` aliasing. Witness: `popw %eax` → `[66,58]`.
- Fix shapes: push 0x66 before Sreg opcodes; add Memory arm with emit_segment_prefix + 0x66 + 8F /0; gate GP with `reg_size==2`.

## encode_bsr_bsf_16 (i686 system encoder)

- Opcode: `0x66` + `0F BC /r` (bsfw) or `0F BD /r` (bsrw); AT&T `bsfw/bsrw src, dst` with dest in ModRM.reg.
- Non-segment r16×r16 and mem→r16 forms match llvm-mc `-triple=i686`.
- Metamorphic: bsfw/bsrw r16 = `[0x66] ‖` bsfl/bsrl r32 for corresponding register pairs.
- Segment override must precede `0x66` (llvm-mc: `bsfw %es:(%eax), %bx` → `[26,66,0f,bc,18]`).
- Known defects: (1) memory arm does not call `emit_segment_prefix` before opcode (same class as lmsw/invlpg/pop16/…). Witness: `bsfw %es:(%eax), %bx` → SUT omits `0x26`. (2) register arm accepts r32/r8 via `reg_num` aliasing. Witness: `bsfw %eax, %bx` → `[66,0f,bc,d8]`.
- Fix shapes: `emit_segment_prefix(mem)` before `0x66` on mem arm; gate both regs with `reg_size==2`.

## encode_mov_infer_size (i686 gp_integer)

- Unsuffixed `mov` (mod.rs:163) → encode_mov_infer_size; size from first Register else second else default 4; then encode_mov.
- Same-width GP forms (RR / Imm→Reg / Mem↔Reg) at widths 1/2/4 match llvm-mc `-triple=i686`.
- Metamorphic: unambiguous `mov` ≡ `movb`/`movw`/`movl` for the inferred width.
- CR/Sreg pairs through unsuffixed `mov` still match llvm-mc (specialized paths inside encode_mov).
- Known defects: (1) no-register form (imm→mem) defaults to size 4 instead of rejecting ambiguous; (2) mismatched GP widths take first-reg size instead of Err.
- Fix shapes: reject `_ =>` default; when both ops are GP regs require equal reg_size before encode_mov.

## encode_mov_mem_reg (i686 gp_integer encoder)

- Opcode: 8A (size1) / 8B (else) + optional 0x66 for size2; ModRM.reg = dest; memory via encode_modrm_mem.
- Non-segment base/disp/SIB/abs forms (except moffs-preferred abs→eAX) match llvm-mc `-triple=i686`.
- Metamorphic: same mem+GP → load (8A/8B) and store (88/89) share prefixes+ModRM/SIB/disp; only opcode differs.
- moffs note: llvm-mc may emit A0/A1 for abs→al/ax/eax; SUT uses general 8A/8B form — both valid; differential skips moffs when llvm chooses it.
- Known defects: (1) segment arm only accepts fs/gs and Errs on es/cs/ss/ds (should call emit_segment_prefix); (2) no reg_size vs mnemonic size gate (movl mem,%ax accepted); (3) reg_num aliases xmm/mm/st as GP dest.
- Fix shapes: `self.emit_segment_prefix(mem);`; `if reg_size(&dst.name) != size { return Err(...); }`; reject is_xmm/is_mm/st/ymm dest.
- Harness: encode_mov_mem_reg_pbt.rs; proptest cases=1000; llvm-mc /home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding.

## encode_mov_reg_mem (i686 gp_integer encoder)

- Opcode: 88 (size1) / 89 (else) + optional 0x66 for size2; ModRM.reg = src; memory via encode_modrm_mem.
- Non-segment base/disp/SIB/abs forms (except moffs-preferred eAX→abs) match llvm-mc `-triple=i686`.
- Metamorphic: same mem+GP → store (88/89) and load (8A/8B) share prefixes+ModRM/SIB/disp; only opcode differs.
- moffs note: llvm-mc may emit A2/A3 for al/ax/eax→abs; SUT uses general 88/89 form — both valid; differential skips moffs when llvm chooses it.
- Known defects: (1) segment arm only accepts fs/gs and Errs on es/cs/ss/ds (should call emit_segment_prefix); (2) no reg_size vs mnemonic size gate (movl %ax,mem accepted); (3) reg_num aliases xmm/mm/st as GP src.
- Fix shapes: `self.emit_segment_prefix(mem);`; `if reg_size(&src.name) != size { return Err(...); }`; reject is_xmm/is_mm/st/ymm src.
- Harness: encode_mov_reg_mem_pbt.rs; proptest cases=1000; llvm-mc /home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding.

## encode_mov_imm_mem (i686 gp_integer encoder)

- Opcode: C6 (size=1) / C7 (else) /0 + optional leading 0x66 when size=2; trailing LE imm of width bytes.
- Non-segment base/disp/SIB/abs integer-imm forms match llvm-mc `-triple=i686`.
- movl $sym / $sym+N: R_386_32 reloc + 4 zero imm bytes (llvm-mc A,A,A,A).
- Metamorphic: same mem, two integer imms share prefix+opcode+ModRM+SIB+disp; only imm trail differs.
- Known defects: (1) no segment prefix at all — even fs/gs omitted (worse than encode_mov_reg_mem fs/gs-only). Witness: `movb $0, %es:(%eax)` → `[c6,00,00]` vs `[26,c6,00,00]`; `movl $1, %fs:(%eax)` omits 0x64. (2) symbol imm rejected for size 1/2 (error string documents 32-bit only); llvm-mc accepts FK_Data_1/2.
- Fix shapes: `self.emit_segment_prefix(mem);` before 0x66/C6/C7; support size 1/2 symbol with narrow reloc + placeholder bytes.

## encode_movsx (i686 gp_integer encoder)

- Opcodes: 0F BE (src byte) / 0F BF (src word); optional leading 0x66 when dst_size==2 (movsbw).
- Dispatch: movsbl→(1,4), movsbw→(1,2), movswl→(2,4) via encoder/mod.rs:174-176.
- RR and non-segment mem base/disp/SIB/abs forms match llvm-mc `-triple=i686`.
- Metamorphic: same operands movsx vs movzx share prefixes+ModRM/SIB/disp; only opcode lo differs (BE↔B6, BF↔B7).
- Known defects: (1) no segment prefix at all on mem arm — emit_segment_prefix never called; (2) no reg_size vs mnemonic size gate; (3) reg_num aliases xmm/mm/st as GP.
- Fix shapes: `self.emit_segment_prefix(mem);` before 66/opcode on mem arm; `if reg_size(&name) != size { return Err(...); }`; reject is_xmm/is_mm/st/ymm.
- Harness: encode_movsx_pbt.rs; proptest cases=1000; llvm-mc /home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding.

## encode_movzx (i686 gp_integer encoder)

- Opcodes: 0F B6 (src byte) / 0F B7 (src word); optional leading 0x66 when dst_size==2 (movzbw).
- Dispatch: movzbl→(1,4), movzwl→(2,4), movzbw→(1,2) via encoder/mod.rs:179-181.
- RR and non-segment mem base/disp/SIB/abs forms match llvm-mc `-triple=i686`.
- Metamorphic: same operands movzx vs movsx share prefixes+ModRM/SIB/disp; only opcode lo differs (B6↔BE, B7↔BF).
- Known defects: (1) no segment prefix at all on mem arm — emit_segment_prefix never called; (2) no reg_size vs mnemonic size gate; (3) reg_num aliases xmm/mm/st as GP.
- Fix shapes: `self.emit_segment_prefix(mem);` before 66/opcode on mem arm; `if reg_size(&name) != size { return Err(...); }`; reject is_xmm/is_mm/st/ymm.
- Harness: encode_movzx_pbt.rs; proptest cases=1000; llvm-mc /home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding.

## encode_lea (i686 gp_integer encoder)

- Opcode: 0x8D /r; AT&T `leal mem, %dst` with ModRM.reg=dst; dispatch `leal|lea` → encode_lea(ops, 4) at encoder/mod.rs:186 (no `leaw` yet).
- Non-segment base/disp/SIB/abs forms match llvm-mc `-triple=i686`.
- Metamorphic: same mem+dst → lea Mod+RM/SIB/disp equals movl mem→reg; only opcode differs (8D vs 8B).
- Known defects: (1) does not call `emit_segment_prefix` before 0x8D (witness `leal %es:(%eax), %eax` → omits 0x26); (2) dest gated only by `reg_num`, so xmm/mm/st/r8/r16 under `leal` are accepted and aliased to GP encodings; (3) `_size` ignored (no 0x66 path even if leaw were dispatched).
- Fix shapes: `self.emit_segment_prefix(mem);` before `bytes.push(0x8D)`; gate dest with `reg_size` / reject non-GP; honor size==2 with leading 0x66 when leaw is wired.
- Harness: encode_lea_pbt.rs; proptest cases=1000; llvm-mc /home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding.

## encode_push (i686 gp_integer encoder)

- Dispatch: `pushl|push` → encode_push (mod.rs:191); `pushw` → encode_push16 (immediates only).
- Happy path matches llvm-mc i686: r32 short 50+rd; imm8 6A ib (|v|≤127); imm32 68 id; mem FF /6 (ESP/EBP/SIB/abs).
- Symbol immediate: 0x68 + R_386_32 reloc + four zero bytes.
- Known defects (this campaign): (1) memory arm does not call `emit_segment_prefix` (x86-64 sibling does); (2) register arm accepts xmm/mm/r8 via `reg_num` alias as 50+n; (3) no Sreg PUSH table (sibling encode_pop has POP Sreg); (4) r16 via mnemonic `push` missing 0x66 (encodes as r32).
- Fix shapes: `emit_segment_prefix(mem)` before 0xFF; gate register class/size; Sreg table ES=06/CS=0E/SS=16/DS=1E/FS=0FA0/GS=0FA8; emit 0x66 when reg_size==2.
- llvm-mc: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases=1000; harness `encode_push_pbt.rs`.

## encode_push16 (i686 gp_integer encoder)

- Dispatch: `pushw` → encode_push16 (mod.rs:196).
- Imm integer path matches llvm-mc i686: leading 0x66; imm8 `6A ib` when v∈[-128,127]; else `68` + (v as i16) LE (truncates OOR i16 the same way llvm-mc does).
- Metamorphic: pushw imm8 = `[0x66] ‖` pushl imm8.
- Negative: arity ≠1 → Err; r32/r8 → Err (llvm-mc also rejects).
- Known defects (this campaign): (1) no r16 Register arm — `pushw %ax` → Err vs `[66,50]`; (2) no Sreg table — `pushw %es` → Err vs `[66,06]`; (3) no Memory arm — `pushw (%eax)` → Err vs `[66,ff,30]`, and segmented forms need `emit_segment_prefix` before 0x66.
- Fix shapes: r16 arm `0x66; 0x50+n` with `reg_size==2`; Sreg table with 0x66 + classic opcodes; Memory: `emit_segment_prefix` + 0x66 + FF /6.
- llvm-mc: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases=1000; harness `encode_push16_pbt.rs`.

## encode_pop (i686 gp_integer encoder)

- Dispatch: `popl|pop` → encode_pop (mod.rs:196); `popw` → encode_pop16 (separate).
- r32 short form 0x58+n and Sreg forms (es=07, ss=17, ds=1F, fs=0F A1, gs=0F A9; cs rejected) match llvm-mc `-triple=i686`.
- Bare memory form 8F /0 (base/disp/SIB/abs, no segment) matches llvm-mc.
- Metamorphic: `pop` mnemonic alias equals `popl` for r32.
- Known defects (this campaign): (1) memory arm does not call `emit_segment_prefix` before 0x8F (x86-64 sibling does). Witness: `popl %es:(%eax)` → SUT `[8f,00]` vs mc `[26,8f,00]`; `popl %fs:(%eax)` omits 0x64. (2) non-segment register arm accepts r8/r16/xmm via `reg_num` aliasing (`popl %xmm0`/`%al`/`%ax` → `[0x58]`).
- Fix shapes: `self.emit_segment_prefix(mem);` before `push(0x8F)`; gate GP arm with `reg_size==4` and reject xmm/mm/st.
- proptest cases=1000; harness `encode_pop_pbt.rs`.

## encode_test (i686 gp_integer)

- Dispatch: `testl|testw|testb|test` → encode_test (mod.rs:214); size from mnemonic_size_suffix (default 4).
- Happy path matches llvm-mc i686: RR 84/85 (+0x66 for *w); Imm→Reg A8/A9 short on AL/AX/EAX else F6/F7 /0; Imm→Mem bare F6/F7 /0 + modrm.
- Metamorphic intended: segmented Imm→Mem = seg_prefix ‖ bare Imm→Mem (currently broken — no emit_segment_prefix).
- Known defects (this campaign): (1) no Reg→Mem arm — `testl %eax, (%ebx)` → Err("unsupported test operands") vs `[85,03]`; x86-64 sibling has the arm. (2) Imm→Mem skips `emit_segment_prefix` (witness `testl $5, %es:(%eax)` omits 0x26). (3) RR accepts mismatched width via reg_num (`testl %ax, %ebx` → same as eax). (4) RR accepts xmm/mm/st via reg_num (`testb %al, %xmm0` → testb %al,%al).
- Fix shapes: add `(Register, Memory)` with emit_segment_prefix + 84/85 + encode_modrm_mem; `emit_segment_prefix(mem)` on Imm→Mem; gate `reg_size == mnemonic size` and GP-only.
- proptest cases=1000; harness `encode_test_pbt.rs`; llvm-mc `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.

## encode_imul (i686 gp_integer)

- Dispatch: `imull|imul` → encode_imul(ops, 4); `imulw` → encode_imul(ops, 2) (mod.rs:219/818). No `imulb` dispatch.
- Forms: 1-op via encode_unary_rm /5 (F6/F7); 2-op RR/Mem→Reg 0F AF; Imm→Reg and 3-op Imm,Reg/Mem,Reg via 6B (imm8) / 69 (imm16/32).
- Bare 32-bit forms (no segment, size=4) match llvm-mc `-triple=i686`.
- Known defects (this campaign): (1) 2/3-op arms ignore `size` — no 0x66 for imulw; 0x69 always emits imm32 not imm16. Witness: `imulw %ax, %bx` → `[0f,af,d8]` vs `[66,0f,af,d8]`; `imulw $300, %ax` → 4-byte imm. (2) Mem→Reg and Imm,Mem,Reg skip `emit_segment_prefix`. Witness: `imull %es:(%eax), %ebx` omits 0x26. (3) 1-op mem via encode_unary_rm also skips segment prefix. (4) RR accepts non-GP via reg_num (`imull %xmm0, %eax` → `[0f,af,c0]`).
- Fix shapes: `if size==2 { push 0x66 }` on all 2/3-op arms; imm16 for 0x69 when size==2; `emit_segment_prefix(mem)` before opcode on mem arms and in encode_unary_rm; gate `reg_size == size` + GP-only.
- proptest cases=1000; harness `encode_imul_pbt.rs`; llvm-mc `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.

## encode_alu (i686 gp_integer)

- Dispatches add/or/adc/sbb/and/sub/xor/cmp (*b/*w/*l) via mod.rs:204-211 → encode_alu(alu_op 0..7).
- RR and bare mem forms (no segment) match llvm-mc `-triple=i686` for same-width GP.
- EAX short form for large imm (0x05+op*8) matches llvm-mc; sign-ext imm8 via 0x83 matches.
- Known defects (this campaign): (1) memory arms do not call `emit_segment_prefix` (x86-64 sibling does). Witness: `addl %ebx, %es:(%eax)` → SUT `[01,18]` vs mc `[26,01,18]`. (2) size==1 imm→reg always uses 0x80 /r — never AL short form 0x04+op*8 (dead `0x04` branch at line 458). Witness: `addb $1, %al` → `[80,c0,01]` vs `[04,01]`. (3) RR accepts mismatched width via reg_num (`addl %ax, %ebx` → same as eax). (4) RR accepts xmm/mm/st via reg_num (`addb %al, %xmm0` → addb %al,%al).
- Fix shapes: `self.emit_segment_prefix(mem);` before opcode on every mem arm; AL short form when size==1 && dst_num==0; gate with `reg_size == mnemonic size` and GP-only check.
- proptest cases=1000; llvm-mc `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- Residual untested (reloc-oracle hard): GOTPC `_GLOBAL_OFFSET_TABLE_`, SymbolDiff, Label-as-memory arms.

## encode_double_shift (i686 gp_integer.rs:920)
- Opcodes: SHLD Imm `0F A4 /r ib`, CL `0F A5 /r`; SHRD Imm `0F AC /r ib`, CL `0F AD /r`; ModRM.reg=src, r/m=dst.
- Dispatch: mod.rs shldl|shld → opc 0xA4 size=4; shrdl|shrd → 0xAC size=4. Aliases share encoding.
- GP r32 Imm/CL RR forms match llvm-mc `-triple=i686`.
- Known defects: (1) no memory destination arms (Intel r/m32); (2) Imm `*count as u8` truncates out-of-Imm8; (3) no GP-class gate (xmm via reg_num); (4) no width gate (r16/r8 alias); (5) `_size` unused (no 0x66 path).
- Fix shapes: Imm/CL+Reg+Mem with emit_segment_prefix+encode_modrm_mem; Imm8 range check; is_xmm/is_mm + reg_size==size gates.
