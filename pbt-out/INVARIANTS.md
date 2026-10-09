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

