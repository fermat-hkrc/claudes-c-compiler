# Confirmed invariants (encode_rev16)

- Valid REV16 Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `rev16 w0, w1` = 0x5ac00420.
- Success-path word is ARM Data-processing (1 source) REV16: sf 1 0 11010110 00000 000001 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000001<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000001.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `rev16 w0, w1` = 0x5ac00420; `rev16 x0, x1` = 0xdac00420; `rev16 wzr, wzr` = 0x5ac007ff; `rev16 xzr, xzr` = 0xdac007ff; `rev16 lr, x1` = 0xdac0043e; `rev16 x0, xzr` = 0xdac007e0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_rev16)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `rev16 w0, w1` = 0x5ac00420.
- ARM ARM Data-processing (1 source) REV16: REV16 <Wd>, <Wn> / REV16 <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000001 Rn Rd; register 31 is ZR not SP. Vector form is Advanced SIMD two-register miscellaneous REV16, dispatched to encode_neon_two_misc, not encode_rev16.
- Dispatch: encoder/mod.rs:576-578 scalar rev16 => encode_rev16; NEON RegArrangement => encode_neon_two_misc.
- Callers: encoder dispatch only.
- Sibling encode_neon_two_misc is the vector form (not a differential sibling for scalar). Sibling encode_rev/encode_rev32/encode_rbit/encode_clz/encode_cls are different opcodes.
- encode_rev16 does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_rev16 (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_rev16_*.md.

# Confirmed invariants (encode_rbit)

- Valid RBIT Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `rbit w0, w1` = 0x5ac00020.
- Direct-call NEON path RBIT Vd.8b/16b, Vn.8b/16b matches llvm-mc (1000 cases). Dispatch routes RegArrangement to encode_neon_rbit, not encode_rbit.
- Success-path scalar word is ARM Data-processing (1 source) RBIT: sf 1 0 11010110 00000 000000 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000000.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `rbit w0, w1` = 0x5ac00020; `rbit x0, x1` = 0xdac00020; `rbit wzr, wzr` = 0x5ac003ff; `rbit xzr, xzr` = 0xdac003ff; `rbit lr, x1` = 0xdac0003e; `rbit x0, xzr` = 0xdac003e0; `rbit v0.8b, v1.8b` = 0x2e605820; `rbit v0.16b, v1.16b` = 0x6e605820.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_rbit)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `rbit w0, w1` = 0x5ac00020.
- ARM ARM Data-processing (1 source) RBIT: RBIT <Wd>, <Wn> / RBIT <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000000 Rn Rd; register 31 is ZR not SP. Vector form is Advanced SIMD two-register miscellaneous RBIT, T in {8B,16B}.
- Dispatch: encoder/mod.rs:902-909 scalar rbit => encode_rbit; NEON RegArrangement => encode_neon_rbit.
- Callers: encoder dispatch only.
- Sibling encode_neon_rbit is the vector form (not a differential sibling for scalar). Sibling encode_clz/encode_cls/encode_rev are different opcodes.
- encode_rbit does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_rbit (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / neon).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_rbit_*.md.

# Confirmed invariants (encode_fp_arith)

- Valid FADD/FSUB/FMUL/FDIV/FMAX/FMIN/FMAXNM/FMINNM Sd,Sn,Sm / Dd,Dn,Dm including s31/d31 and uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `fadd s0, s1, s2` = 0x1e222820.
- Success-path word is ARM Floating-point data-processing (2 source): 00011110 ftype 1 Rm opcode 10 Rn Rd with ftype 00=S / 01=D. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(rm<<16)|(opcode<<12)|(0b10<<10)|(rn<<5)|rd. opcode 0000=FMUL, 0001=FDIV, 0010=FADD, 0011=FSUB, 0100=FMAX, 0101=FMIN, 0110=FMAXNM, 0111=FMINNM.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Rm+1 increments bits[20:16] only; S vs D xor = 1<<22; FADD xor FSUB = 1<<12 (1000 cases).
- Fewer than 3 operands, non-register kinds (Imm/Symbol/Label/Mem/Cond/Shift), and invalid names (foo/s32/empty/r0) always Err (1000 cases).
- Known-answer: `fadd s0, s1, s2` = 0x1e222820; `fadd d0, d1, d2` = 0x1e622820; `fsub s0, s1, s2` = 0x1e223820; `fmul s0, s1, s2` = 0x1e220820; `fdiv s0, s1, s2` = 0x1e221820; `fmax s0, s1, s2` = 0x1e224820; `fmin s0, s1, s2` = 0x1e225820; `fmaxnm s0, s1, s2` = 0x1e226820; `fminnm s0, s1, s2` = 0x1e227820; `fadd s31, s31, s31` = 0x1e3f2bff; `fadd h0, h1, h2` = 0x1ee22820 (llvm-mc fp16).
- Extra operand, mixed S/D, GPR, SP/WSP, Q/V/B, and H (wrong ftype) currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_fp_arith)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fadd s0, s1, s2` = 0x1e222820.
- ARM ARM Floating-point data-processing (2 source): M=0 S=0 11110 ftype 1 Rm opcode 10 Rn Rd. ftype 00=S 01=D 11=H. Register 31 is a valid S/D/H index.
- Dispatch: encoder/mod.rs:377-404 scalar fadd/fsub/fmul/fdiv/fmax/fmin/fmaxnm/fminnm => encode_fp_arith; RegArrangement => encode_neon_float_three_same.
- Callers: encoder dispatch only.
- Sibling encode_neon_float_three_same is the vector form (not a differential sibling).
- encode_fp_arith does not check operands.len() (extra ignored); takes ftype from Rd prefix only (`starts_with('d')` else 00); parse_reg_num maps sp/wsp/x/w/q/v/h/b.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fp_arith (invalid-name).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fp_arith_*.md.

# Confirmed invariants (encode_fmov)

- Valid FMOV Sd,Sn / Dd,Dn / Sd,Wn / Dd,Xn / Wd,Sn / Xd,Dn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `fmov s0, s1` = 0x1e204020 and `fmov s0, w1` = 0x1e270020.
- Success-path FP-to-FP word is ARM FMOV (register): 00011110 ftype 1 000000 10000 Rn Rd with ftype 00=S / 01=D. Equivalently w = (0b00011110<<24)|(ftype<<22)|(0b100000<<16)|(0b10000<<10)|(rn<<5)|rd.
- Success-path GP↔FP word is ARM FMOV (general): sf 00 11110 ftype 1 00 opcode 000000 Rn Rd with opcode 111 GP→FP / 110 FP→GP; sf/ftype 0/00 for S/W and 1/01 for D/X.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D xor = 1<<22; GP→FP xor FP→GP = 1<<16 (1000 cases).
- Fewer than 2 operands, non-register kinds (Symbol/Label/Mem/Cond/Shift/Expr), and invalid names (foo/s32/empty/r0) always Err (1000 cases).
- Known-answer: `fmov s0, s1` = 0x1e204020; `fmov d0, d1` = 0x1e604020; `fmov s0, w1` = 0x1e270020; `fmov d0, x1` = 0x9e670020; `fmov w0, s1` = 0x1e260020; `fmov x0, d1` = 0x9e660020; `fmov s0, wzr` = 0x1e2703e0; `fmov d0, lr` = 0x9e6703c0; `fmov h0, h1` = 0x1ee04020 (llvm-mc fp16); `fmov x0, v0.d[1]` = 0x9eae0000.
- Extra operand, mixed S/D, size-mismatched GP/FP, Q/V/B, SP/WSP, H (wrong ftype), and V.D[1] currently encode or reject incorrectly (see bugs).

## Environment (encode_fmov)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fmov s0, s1` = 0x1e204020.
- ARM ARM FMOV (register): 0 00 11110 ftype 1 000000 10000 Rn Rd. FMOV (general): sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:376 "fmov" => encode_fmov.
- Callers: encoder dispatch only. Codegen emits S/W and D/X register forms (float_ops.rs, cast_ops.rs, alu.rs); asm_emitter.rs:205 notes "fmov requires d/s register form, not v".
- encode_fmov checks operands.len() < 2 only (extra ignored); is_fp_reg includes q/v/h/b and "sp"; ftype is 01 iff a name starts with 'd' else 00; GP-FP path ignores GP width; RegLane is not matched.
- FMOV (immediate) is a documented TODO (fp_scalar.rs:14-15) — Design Caveat, not a bug.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fmov (nonreg / invalid-name / V.D[1]).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fmov_*.md.

# Confirmed invariants (encode_extr)

- Valid EXTR Wd,Wn,Wm / Xd,Xn,Xm with 0 <= lsb < R (R=32/64), including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `extr w0, w1, w2, #0` = 0x13820020.
- Success-path word is ARM Extract EXTR: sf 00 100111 N 0 Rm imms Rn Rd with N=sf, imms=lsb. Equivalently w = (sf<<31)|(0b00100111<<23)|(sf<<22)|(rm<<16)|(lsb<<10)|(rn<<5)|rd. bits[30:23]=00100111; bit21=0.
- Algebraic alias: encode_extr(Rd,Rn,Rn,#lsb) = llvm-mc("ror Rd, Rn, #lsb") (ARM ROR immediate alias when Rn=Rm) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Rm+1 increments bits[20:16] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds at the wrong slot, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `extr w0, w1, w2, #0` = 0x13820020; `extr w0, w1, w2, #1` = 0x13820420; `extr w0, w1, w2, #31` = 0x13827c20; `extr x0, x1, x2, #0` = 0x93c20020; `extr x0, x1, x2, #63` = 0x93c2fc20; `extr wzr, wzr, wzr, #0` = 0x139f03ff; `extr lr, x1, x2, #8` = 0x93c2203e; `ror w0, w1, #1` = 0x13810420.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb currently encode or panic instead of Err (see bugs).

## Environment (encode_extr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `extr w0, w1, w2, #0` = 0x13820020.
- ARM ARM Extract EXTR: EXTR <Wd>, <Wn>, <Wm>, #<lsb> / EXTR <Xd>, <Xn>, <Xm>, #<lsb>. Encoding sf 00 100111 N 0 Rm imms Rn Rd; N=sf; 0 <= lsb <= 31 (W) / 63 (X); register 31 is ZR not SP. ROR (immediate) is the alias of EXTR when Rn=Rm.
- Dispatch: encoder/mod.rs:894 "extr" => encode_extr.
- Callers: encoder dispatch only.
- Sibling encode_shift ROR is a different mnemonic (3-operand shift) — not a differential sibling; used only as ARM alias when Rn=Rm.
- encode_extr does not check operands.len() (extra ignored); takes sf from Rd without checking Rn/Rm width or FP prefix; parse_reg_num maps sp/wsp to 31; lsb is `as u32` with no ARM range check (negative panics on `lsb << 10` in debug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_extr (arity / extra / SP / mixed W-X / FP / lsb / nonreg / invalid-name / alt-spellings).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_extr_*.md.

# Confirmed invariants (encode_clz)

- Valid CLZ Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `clz w0, w1` = 0x5ac01020.
- Success-path word is ARM Data-processing (1 source) CLZ: sf 1 0 11010110 00000 000100 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000100<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000100.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `clz w0, w1` = 0x5ac01020; `clz x0, x1` = 0xdac01020; `clz wzr, wzr` = 0x5ac013ff; `clz xzr, xzr` = 0xdac013ff; `clz lr, x1` = 0xdac0103e; `clz x0, xzr` = 0xdac013e0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_clz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `clz w0, w1` = 0x5ac01020.
- ARM ARM Data-processing (1 source) CLZ: CLZ <Wd>, <Wn> / CLZ <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000100 Rn Rd; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:573-575 scalar clz => encode_clz; NEON RegArrangement => encode_neon_two_misc.
- Callers: encoder dispatch only.
- Sibling encode_cls is a different opcode (000101, count leading sign bits) — not a differential sibling.
- encode_clz does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_clz (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_clz_*.md.

# Confirmed invariants (encode_cls)

- Valid CLS Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `cls w0, w1` = 0x5ac01420.
- Success-path word is ARM Data-processing (1 source) CLS: sf 1 0 11010110 00000 000101 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000101<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000101.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `cls w0, w1` = 0x5ac01420; `cls x0, x1` = 0xdac01420; `cls wzr, wzr` = 0x5ac017ff; `cls xzr, xzr` = 0xdac017ff; `cls lr, x1` = 0xdac0143e; `cls x0, xzr` = 0xdac017e0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_cls)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `cls w0, w1` = 0x5ac01420.
- ARM ARM Data-processing (1 source) CLS: CLS <Wd>, <Wn> / CLS <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000101 Rn Rd; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:570-572 scalar cls => encode_cls; NEON RegArrangement => encode_neon_two_misc.
- Callers: encoder dispatch only.
- Sibling encode_clz is a different opcode (000100, count leading zeros) — not a differential sibling.
- encode_cls does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_cls (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_cls_*.md.

# Confirmed invariants (encode_cas)

- Valid CAS/CASA/CASAL/CASL on matching W or X Rs/Rt with [Xn|SP], and CASB/CASH (plus acquire/release) on W Rs/Rt with [Xn|SP], including wzr/xzr and sp-as-base, matches llvm-mc `-triple=aarch64 -mattr=+lse -show-encoding` (1000 cases). gas aarch64-linux-gnu-as -march=armv8-a+lse agrees on `cas x0, x1, [x2]` = 0xc8a07c41.
- Success-path word is ARM CAS: size 001000 1 L 1 Rs o0 11111 Rn Rt. size 00=byte 01=half 10=word 11=doubleword; L=1 for CASA/CASAL; o0=1 for CASL/CASAL; bits[29:24]=001000; bit23=1; bit21=1; bits[14:10]=11111. Equivalently w = (size<<30)|(0b001000<<24)|(1<<23)|(L<<22)|(1<<21)|(rs<<16)|(o0<<15)|(0b11111<<10)|(rn<<5)|rt.
- Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Rs+1 increments bits[20:16] only; CASA xor CAS = 1<<22; CASL xor CAS = 1<<15; CASAL xor CAS = (1<<22)|(1<<15); uppercase mnemonic matches lowercase (1000 cases).
- Fewer than 3 operands, non-Mem third operand (Imm/Symbol/Cond/pre/post/reg-offset), and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `cas x0, x1, [x2]` = 0xC8A07C41; `cas w0, w1, [x2]` = 0x88A07C41; `casa w0, w1, [x2]` = 0x88E07C41; `casl w0, w1, [x2]` = 0x88A0FC41; `casal w0, w1, [x2]` = 0x88E0FC41; `casb w0, w1, [x2]` = 0x08A07C41; `cash w0, w1, [x2]` = 0x48A07C41; `cas xzr, xzr, [sp]` = 0xC8BF7FFF.
- Extra operand, SP/WSP as Rs/Rt, XZR/x31/WZR as base, W/WSP as base, mixed W/X, FP/SIMD prefixes, CASB/CASH with X, and nonzero Mem offset currently encode instead of Err (see bugs).
- gas accepts optional `#0` offset (ARM `{,#0}`); llvm-mc 15 rejects `#0`. At the encode_cas boundary `[Xn]` and `[Xn, #0]` are the same Operand::Mem{offset:0}, so the #0 form is not distinguishable here.

## Environment (encode_cas)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+lse -show-encoding. gas aarch64-linux-gnu-as -march=armv8-a+lse agrees on `cas x0, x1, [x2]` = 0xc8a07c41.
- ARM ARM Compare and Swap: size 001000 1 L 1 Rs o0 11111 Rn Rt. Rs/Rt are ZR not SP at 31; Rn is SP not ZR at 31. CASB/CASH require W registers.
- Dispatch: encoder/mod.rs:920-922 cas/casa/casal/casl/casb/casab/casalb/caslb/cash/casah/casalh/caslh => encode_cas.
- Callers: encoder dispatch only.
- Sibling encode_swp is a different LSE class (not a differential sibling).
- encode_cas checks operands.len() < 3 only (extra operands ignored); takes size from Rs (or b/h suffix) without checking Rt width or FP prefix; Mem offset is discarded (`base, ..`).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_cas (arity / extra / SP / XZR-base / W-base / mixed W-X / FP / casb-X / nonzero offset / non-mem / invalid-name / alt-spellings).
- Eight failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_cas_*.md.

# Confirmed invariants (encode_bfxil)

- Valid BFXIL Wd,Wn / Xd,Xn with 0 <= lsb < R and 1 <= width <= R-lsb (R=32/64), including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `bfxil w0, w1, #0, #1` = 0x33000020.
- Success-path word is BFM: sf 01 100110 N immr imms Rn Rd with N=sf, immr=lsb, imms=lsb+width-1. Equivalently w = (sf<<31)|(0b01<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd.
- Algebraic alias: encode_bfxil(Rd,Rn,#lsb,#width) = encode_bfm(Rd,Rn,#lsb,#(lsb+width-1)) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds at the wrong slot, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `bfxil w0, w1, #0, #1` = 0x33000020; `bfxil w0, w1, #1, #1` = 0x33010420; `bfxil x0, x1, #1, #8` = 0xb3412020; `bfxil wzr, wzr, #31, #1` = 0x331f7fff; `bfxil x0, xzr, #63, #1` = 0xb37fffe0; `bfxil lr, x1, #8, #16` = 0xb3485c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_bfxil)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `bfxil w0, w1, #0, #1` = 0x33000020.
- ARM ARM Bitfield Move BFXIL alias of BFM: BFXIL Wd, Wn, #lsb, #width <=> BFM Wd, Wn, #lsb, #(lsb+width-1) with 0 <= lsb < 32 and 1 <= width <= 32-lsb (64-bit analog). Encoding sf 01 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:893 "bfxil" => encode_bfxil.
- Callers: encoder dispatch only.
- Sibling encode_bfm is the raw form (used as algebraic alias after ARM mapping, not as a differential sibling).
- encode_bfxil computes imms = lsb + width - 1 without checking ARM bounds; width=0 and lsb=0 panics in debug (subtract overflow); large lsb+width panics (add overflow).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_bfxil (arity / extra / SP / mixed W-X / FP / lsb-width / nonreg / invalid-name / alt-spellings).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_bfxil_*.md.

# Confirmed invariants (encode_bfi)

- Valid BFI Wd,Wn / Xd,Xn with 0 <= lsb < R and 1 <= width <= R-lsb (R=32/64), including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `bfi w0, w1, #0, #1` = 0x33000020.
- Success-path word is BFM: sf 01 100110 N immr imms Rn Rd with N=sf, immr=(-lsb MOD R), imms=width-1. Equivalently w = (sf<<31)|(0b01<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd.
- Algebraic alias: encode_bfi(Rd,Rn,#lsb,#width) = encode_bfm(Rd,Rn,#(-lsb MOD R),#(width-1)) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds at the wrong slot, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `bfi w0, w1, #0, #1` = 0x33000020; `bfi w0, w1, #1, #1` = 0x331f0020; `bfi x0, x1, #1, #8` = 0xb37f1c20; `bfi wzr, wzr, #31, #1` = 0x330103ff; `bfi x0, xzr, #63, #1` = 0xb34103e0; `bfi lr, x1, #8, #16` = 0xb3783c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_bfi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `bfi w0, w1, #0, #1` = 0x33000020.
- ARM ARM Bitfield Move BFI alias of BFM: BFI Wd, Wn, #lsb, #width <=> BFM Wd, Wn, #(-lsb MOD 32), #(width-1) with 0 <= lsb < 32 and 1 <= width <= 32-lsb (64-bit analog). Encoding sf 01 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. llvm-mc prefers BFXIL in disassembly when immr=0 (lsb=0) because the encodings coincide.
- Dispatch: encoder/mod.rs:892 "bfi" => encode_bfi.
- Callers: encoder dispatch only.
- Sibling encode_bfm is the raw form (used as algebraic alias after ARM mapping, not as a differential sibling).
- encode_ubfiz uses wrapping_sub for immr; encode_bfi uses non-wrapping `reg_width - lsb`, which panics in debug when lsb > reg_width.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_bfi (arity / extra / SP / mixed W-X / FP / lsb-width / nonreg / invalid-name / alt-spellings).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_bfi_*.md.

# Confirmed invariants (encode_neon_aes)

- Valid AES AESE/AESD/AESMC/AESIMC Vd.16b, Vn.16b (including v31, uppercase V/16B) matches llvm-mc `-triple=aarch64 -mattr=+aes -show-encoding` (1000 cases). gas aarch64-linux-gnu-as -march=armv8-a+crypto agrees on `aese v0.16b, v1.16b` = 0x4e284820.
- Success-path word: 0100 1110 0010 1000 opcode 10 Rn Rd. Equivalently w = (0b01001110<<24)|(0b0010100<<17)|(opc<<12)|(0b10<<10)|(rn<<5)|rd. opc AESE=00100 AESD=00101 AESMC=00110 AESIMC=00111; bits[31:24]=01001110; bits[23:17]=0010100; bits[11:10]=10.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; AESD XOR AESE = 1<<12; AESMC XOR AESE = 1<<13 (1000 cases).
- Fewer than 2 operands, non-register dest/src kinds, and invalid names (v32/foo/empty/v/v99/v-1) always Err (1000 cases).
- Known-answer: `aese v0.16b, v1.16b` = 0x4e284820; `aesd v0.16b, v1.16b` = 0x4e285820; `aesmc v0.16b, v1.16b` = 0x4e286820; `aesimc v0.16b, v1.16b` = 0x4e287820; `aese v31.16b, v31.16b` = 0x4e284bff; `aesd v31.16b, v0.16b` = 0x4e28581f; `aesmc v0.16b, v31.16b` = 0x4e286be0; `aesimc v15.16b, v16.16b` = 0x4e287a0f.
- Extra operand, T other than .16b, mismatched T, bare Vn, non-V prefix, and SP/WSP currently encode incorrectly (see bugs).

## Environment (encode_neon_aes)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+aes -show-encoding. gas aarch64-linux-gnu-as -march=armv8-a+crypto agrees on `aese v0.16b, v1.16b` = 0x4e284820.
- ARM ARM Cryptographic AES: 0 1 0 0 1 1 1 0 size 1 01000 opcode 10 Rn Rd. size must be 00 (otherwise unallocated). opcode 00100=AESE 00101=AESD 00110=AESMC 00111=AESIMC. Assembly syntax is only Vd.16B, Vn.16B.
- Dispatch: encoder/mod.rs:747-750 aese/aesd/aesmc/aesimc => encode_neon_aes with opcodes 00100/00101/00110/00111.
- Callers: encoder dispatch only; no ARM codegen emitter of aese/aesd/aesmc/aesimc found (x86 AES-NI is a different ISA).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_neon_aes (arity / extra / T / mismatch / bare Reg / GPR prefix / SP / WSP / nonreg src / invalid-name).
- Six failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_aes_*.md.

# Confirmed invariants (encode_fcvt_precision)

- Valid scalar FCVT Sd|Dd|Hd, Sn|Dn|Hn with dest precision != src precision (including s31/d31/h31, uppercase) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases; half uses -mattr=+fullfp16).
- Success-path word: 0 00 11110 ftype 1 0001 opc 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b0001<<17)|(opc<<15)|(0b10000<<10)|(rn<<5)|rd. ftype 00=S 01=D 11=H source; opc 00=S 01=D 11=H dest; bits[31:24]=00011110; bit21=1; bits[20:17]=0001; bits[14:10]=10000.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S dest vs D dest (H src) flips only bit 15; S src vs D src (H dest) flips only bit 22 (1000 cases).
- Fewer than 2 operands, non-register kinds, invalid names (foo/s32/d32/h32/empty/r0), and GPR/Q/V/B/WSP in either slot always Err (1000 cases).
- Known-answer: `fcvt d0, s1` = 0x1e22c020; `fcvt s0, d1` = 0x1e624020; `fcvt h0, s1` = 0x1e23c020; `fcvt s0, h1` = 0x1ee24020; `fcvt d0, h1` = 0x1ee2c020; `fcvt h0, d1` = 0x1e63c020; `fcvt d31, s31` = 0x1e22c3ff; `fcvt s31, d0` = 0x1e62401f.
- Extra operand, same-precision S/D/H, and SP dest/src currently encode incorrectly (see bugs).

## Environment (encode_fcvt_precision)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fcvt d0, s1` = 0x1e22c020.
- ARM ARM Floating-point data-processing (1 source) FCVT: 0 00 11110 ftype 1 0001 opc 10000 Rn Rd. ftype 00=S 01=D 11=H source; opc 00=S 01=D 11=H dest; ftype==opc is unallocated.
- Dispatch: encoder/mod.rs:460 "fcvt" => encode_fcvt_precision. fcvtzs/fcvtzu/etc. go to encode_fcvt_rounding. fcvtl/fcvtn go to NEON.
- Callers: encoder dispatch; codegen/cast_ops.rs:74-78 emits `fcvt d0, s0` / `fcvt s0, d0`.
- Unlike encode_fcmp / encode_fp_1src, this encoder already selects ftype/opc from s/d/h prefixes, so half-precision conversions match llvm-mc.
- parse_reg_num maps sp to 31 and dest/src first char 's' selects S, so SP encodes as S31 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fcvt_precision (arity / extra / same-precision / GPR / QVB / WSP / SP dest+src / half / nonreg / invalid-name).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fcvt_precision_*.md.

# Confirmed invariants (encode_fcmp)

- Valid scalar FCMP Sn,Sm or Dn,Dm (including s31/d31, uppercase) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid FCMP Sn|Dn, #0.0 via Operand::Imm(0) matches llvm-mc `fcmp Sn|Dn, #0.0` (1000 cases).
- Success-path register word: 0 00 11110 ftype 1 Rm 001000 Rn 00000. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(rm<<16)|(0b001000<<10)|(rn<<5). ftype 00=S 01=D; bits[15:10]=001000; bit21=1; opc=00000.
- Success-path #0.0 word: Rm=00000, opc=01000. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b001000<<10)|(rn<<5)|0b01000.
- Metamorphic: Rn+1 increments bits[9:5] only; Rm+1 increments bits[20:16] only; S vs D flips only bit 22; register (Rm=0) XOR #0.0 = 1<<3 (1000 cases).
- Non-zero Imm, non-register kinds (Imm(1)/Symbol/Label/Mem/Cond/Shift), and invalid names (foo/s32/d32/empty/r0) always Err (1000 cases).
- Known-answer: `fcmp s0, s1` = 0x1e212000; `fcmp d0, d1` = 0x1e612000; `fcmp s0, #0.0` = 0x1e202008; `fcmp d0, #0.0` = 0x1e602008; `fcmp s31, s31` = 0x1e3f23e0; `fcmp d31, d0` = 0x1e6023e0; llvm-mc +fullfp16 `fcmp h0, h1` = 0x1ee12000 / `fcmp h0, #0.0` = 0x1ee02008 (SUT currently 0x1e212000 / 0x1e202008, see bugs).
- One operand, extra operand, mixed S/D (and GPR/SP/QVB), and H registers currently encode incorrectly (see bugs).

## Environment (encode_fcmp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fcmp s0, s1` = 0x1e212000.
- ARM ARM Floating-point compare: 0 00 11110 ftype 1 Rm 001000 Rn opc. ftype 00=S 01=D 11=H; opc 00000=FCMP register, 01000=FCMP #0.0, 10000=FCMPE register, 11000=FCMPE #0.0. Immediate form is only #0.0.
- Dispatch: encoder/mod.rs:439 "fcmp" => encode_fcmp. fccmp is a different mnemonic. fcmpe is not dispatched.
- Callers: encoder dispatch; codegen/comparison.rs:15-19 emits `fcmp s0, s1` / `fcmp d0, d1` (never the #0.0 form).
- Encoder mapping: Operand::Imm(0) <-> `#0.0` (Operand has no float-immediate variant). Parser currently turns textual `#0.0` into Operand::Expr("0.0"), which encode_fcmp rejects via get_reg — a parser/encoder seam, not claimed as this function's contract.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fcmp (arity / extra / mixed S-D / GPR / QVB / SP / half ftype / nonzero imm / nonreg / invalid-name).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fcmp_*.md.

# Confirmed invariants (encode_int_to_float)

- Valid integer SCVTF/UCVTF Sd|Dd, Wn|Xn (including wzr/xzr, w31/x31, lr, uppercase, mixed S/X and D/W) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: sf 00 11110 ftype 1 00 opcode 000000 Rn Rd. Equivalently w = (sf<<31)|(0b11110<<24)|(ftype<<22)|(1<<21)|(opcode<<16)|(rn<<5)|rd. sf 0=W source 1=X source; ftype 00=S dest 01=D dest; opcode 010=SCVTF 011=UCVTF; bits[30:29]=00, bits[20:19]=00 (rmode), bits[15:10]=0, bit21=1.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; W vs X source flips only bit 31; S vs D dest flips only bit 22; SCVTF XOR UCVTF = 1<<16 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/s32/empty/r0) always Err (1000 cases).
- Known-answer: `scvtf s0, w1` = 0x1e220020; `scvtf d0, x1` = 0x9e620020; `ucvtf s0, w1` = 0x1e230020; `ucvtf d0, x1` = 0x9e630020; `scvtf s0, x1` = 0x9e220020; `scvtf d31, xzr` = 0x9e6203ff; `scvtf d0, lr` = 0x9e6203c0; llvm-mc +fullfp16 `scvtf h0, w1` = 0x1ee20020 (SUT currently 0x1e220020, see bugs).
- Extra operand, SP/WSP source, GP dest / FP source / QVB, and H dest currently encode incorrectly (see bugs).

## Environment (encode_int_to_float)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `scvtf s0, w1` = 0x1e220020.
- ARM ARM Conversion between floating-point and integer: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd. For integer SCVTF/UCVTF rmode=00, opcode=010/011. sf 0=W source 1=X source; ftype 00=S 01=D 11=H; register 31 is ZR not SP. Fixed-point form uses bit21=0 plus scale.
- Dispatch: encoder/mod.rs:454-459 ucvtf/scvtf => encode_ucvtf/encode_scvtf => encode_int_to_float. Vector RegArrangement goes to encode_neon_float_two_misc. SIMD-scalar `scvtf s0, s1` = 0x5e21d820 is a different class.
- Callers: encoder dispatch; codegen/cast_ops.rs:53-66 emits `scvtf`/`ucvtf` d0/s0, x0.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_int_to_float (arity / extra / SP src / GP dest / FP source / QVB / H ftype / nonreg / invalid-name).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_int_to_float_*.md.

# Confirmed invariants (encode_fp_1src)

- Valid scalar FRINTN/P/M/Z/A/X/I Sd,Sn or Dd,Dn (including s31/d31, uppercase, all 7 mnemonics) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: 0 00 11110 ftype 1 opcode 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(opcode<<15)|(0b10000<<10)|(rn<<5)|rd. ftype 00=S 01=D; bits[14:10]=10000; bit21=1.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only bit 22; FRINTN XOR FRINTP = 1<<15 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/s32/d32/empty/r0) always Err (1000 cases).
- Known-answer: `frintn s0, s1` = 0x1e244020; `frintn d0, d1` = 0x1e644020; `frintp s0, s1` = 0x1e24c020; `frintz s0, s1` = 0x1e25c020; `frinti s0, s1` = 0x1e27c020; `frintn s31, s31` = 0x1e2443ff; llvm-mc +fullfp16 `frintn h0, h1` = 0x1ee44020 (SUT currently 0x1e244020, see bugs).
- Extra operand, mixed S/D (and GPR/SP/QVB), and H registers currently encode incorrectly (see bugs).

## Environment (encode_fp_1src)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `frintn s0, s1` = 0x1e244020.
- ARM ARM Floating-point data-processing (1 source): M=0 S=0 11110 ftype 1 opcode 10000 Rn Rd. ftype 00=S 01=D 11=H. FRINTN=001000 FRINTP=001001 FRINTM=001010 FRINTZ=001011 FRINTA=001100 FRINTX=001110 FRINTI=001111.
- Dispatch: encoder/mod.rs:414-434 frintn/p/m/z/a/x/i => encode_fp_1src. Vector RegArrangement goes to encode_neon_float_two_misc.
- Callers: encoder dispatch only (no codegen emitter of scalar frint* found).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fp_1src (arity / extra / mixed S-D / GPR / QVB / SP / half ftype / nonreg / invalid-name / uppercase / S vs D / all 7 opcodes).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fp_1src_*.md.

# Confirmed invariants (encode_fcvt_rounding)

- Valid integer FCVT* Wd|Xd, Sn|Dn (including wzr/xzr, w31/x31, lr, uppercase, all 10 mnemonics fcvtzs/zu/as/au/ns/nu/ms/mu/ps/pu) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd. Equivalently w = (sf<<31)|(0b11110<<24)|(ftype<<22)|(1<<21)|(rmode<<19)|(opcode<<16)|(rn<<5)|rd. bits[30:29]=00, bits[15:10]=0, bit21=1.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; W vs X flips only bit 31; S vs D flips only bit 22; FCVTZS XOR FCVTZU = 1<<16 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/s32/empty/r0) always Err (1000 cases).
- Known-answer: `fcvtzs w0, s1` = 0x1e380020; `fcvtzs x0, d1` = 0x9e780020; `fcvtzu w0, s1` = 0x1e390020; `fcvtas w0, s1` = 0x1e240020; `fcvtzs xzr, d0` = 0x9e78001f; `fcvtzs lr, s0` = 0x9e38001e; llvm-mc +fullfp16 `fcvtzs w0, h1` = 0x1ef80020 (SUT currently 0x1e380020, see bugs).
- Extra operand, SP/WSP dest, FP dest / GP source / QVB, and H source currently encode incorrectly (see bugs).

## Environment (encode_fcvt_rounding)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fcvtzs w0, s1` = 0x1e380020.
- ARM ARM Conversion between floating-point and integer: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd. sf 0=W dest 1=X dest; ftype 00=S 01=D 11=H; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:440-453 fcvtzs/fcvtzu/fcvtas/au/ns/nu/ms/mu/ps/pu => encode_fcvt_rounding. fcvtzs/fcvtzu with RegArrangement go to NEON instead.
- Callers: encoder dispatch; codegen/cast_ops.rs emits `fcvtzs`/`fcvtzu`.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fcvt_rounding (arity / extra / SP dest / FP dest / GP source / QVB / H ftype / nonreg / invalid-name).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fcvt_rounding_*.md.

# Confirmed invariants (encode_smulh)

- Valid SMULH Xd, Xn, Xm (including xzr, lr, x31, uppercase, LR/XZR) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: sf=1 op54=00 11011 op31=010 o0=0 Ra=11111; w = 0x9B407C00 | (rm<<16) | (rn<<5) | rd.
- Metamorphic: SMULH XOR UMULH is bit 23 (U); Rd+1 / Rn+1 / Rm+1 update only that field (1000 cases).
- Fewer than 3 operands, non-register operands, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `smulh x0, x1, x2` = 0x9B427C20; `smulh xzr, xzr, xzr` = 0x9B5F7FFF; `smulh lr, x1, x30` = 0x9B5E7C3E; `smulh x0, x1, xzr` = 0x9B5F7C20.
- Extra operand, W registers (including wzr), SP/WSP, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_smulh)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) SMULH: 1 00 11011 010 Rm 0 11111 Rn Rd; 64-bit only; register 31 is XZR not SP.
- Dispatch: encoder/mod.rs:275 "smulh" => encode_smulh.
- Callers: encoder dispatch; no codegen emitter of smulh found (umulh is used in i128_ops.rs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_smulh (arity / extra / W-width / WZR / SP / FP / nonreg / invalid-name / alt-spellings / field independence).

# Confirmed invariants (encode_prfm)

- Valid PRFM (immediate) with named prfop or #imm5 in 0..31, base Xn|SP, pimm = imm12*8 in [0, 32760] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Uppercase prfop/Xn/SP spellings match llvm-mc (1000 cases).
- Success-path unsigned word: size=11 V=0 opc=10; 1111 1001 10 imm12 Rn Rt. Equivalently w = 0xF9800000 | (imm12<<10) | (rn<<5) | prfop.
- Metamorphic: prfop+1 increments Rt only; Rn+1 increments Rn field only; imm12+1 increments imm12 only (1000 cases).
- Fewer than 2 operands, unaligned/negative/too-large pimm, pre/post-index, Imm/Label/Cond address, unknown prfop name, #imm5 outside 0..31, Reg-as-prfop, base foo/x32, invalid index name, and PRFM literal Symbol always Err (1000 cases).
- Known-answer: `prfm pldl1keep, [x0]` = 0xF9800000; `prfm pldl1keep, [x1, #8]` = 0xF9800420; `prfm pldl1strm, [sp, #16]` = 0xF9800BE1; `prfm #31, [x0]` = 0xF980001F; `prfm pldl1keep, [x0, #32760]` = 0xF9BFFC00; llvm-mc `prfm pldl1keep, [x0, x1]` = 0xF8A16800 (SUT currently 0xF9216800, see bugs).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM PRFM (immediate): 1111 1001 10 imm12 Rn Rt; pimm multiple of 8 in [0,32760]. PRFM (register): 11 111 0 00 10 1 Rm option S 10 Rn Rt; option UXTW/LSL/SXTW/SXTX; S amount 0 or 3. Rt is 5-bit prfop. Rn is Xn|SP (31=SP, not XZR).
- Dispatch: encoder/mod.rs:917 "prfm" => encode_prfm.
- Callers: encoder dispatch; inline_asm.rs mentions prfm/prefetch.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- parse_reg_num maps w/wsp/xzr/x31/FP prefixes, so invalid bases encode (see bugs).
- PRFM (register) uses `(0b10 << 23)` instead of `(0b10 << 22)` (see bugs).
- Bare W-index defaults to UXTW (see bugs).
- Shift amount > 0 is encoded as S=1 regardless of 1 vs 3 (see bugs).
- PRFM (literal) Symbol returns Err("not yet supported") — Doc evidence load_store.rs:759-761.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_prfm (arity / extra / W-base / XZR / x31 / FP / offset range / pre/post / unknown prfop / imm5 range / W-index / bad shift / Reg-prfop / invalid names / literal).
- Seven failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_prfm_*.md.

# Confirmed invariants (encode_ldtr_sized)

- Valid LDTRB/LDTRH/STTRB/STTRH Wt, [Xn|SP{, #simm}] with simm in [-256, 255] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Uppercase Wt/Xn/SP/WZR spellings match llvm-mc (1000 cases).
- w31 dest encodes as wzr and matches llvm-mc (1000 cases).
- Success-path word: size 111 V=0 00 opc 0 imm9 10 Rn Rt. size=00 byte / 01 half; opc=01 load / 00 store. Equivalently w = (size<<30) | 0x38000800 | (opc<<22) | ((imm9 as u32 & 0x1FF)<<12) | (rn<<5) | rt.
- Metamorphic: size bit XOR = 1<<30; load XOR store = 1<<22; Rt+1 adds 1; Rn+1 adds 32; imm9+1 only changes bits[20:12] (1000 cases).
- Fewer than 2 operands and non-Mem addressing (pre/post/reg-offset/Imm/Symbol/Label) always Err (1000 cases).
- Known-answer: `ldtrb w0, [x1]` = 0x38400820; `ldtrh w0, [x1]` = 0x78400820; `sttrb w0, [x1]` = 0x38000820; `sttrh w0, [x1]` = 0x78000820; `ldtrb w0, [x1, #-256]` = 0x38500820; `ldtrb w0, [sp, #255]` = 0x384ffbe0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM LDTRB/LDTRH/STTRB/STTRH: size 111 V=0 00 opc 0 imm9 10 Rn Rt. Syntax Wt, [Xn|SP{, #simm}]; simm9 in [-256,255]. Register 31 is WZR for Rt, SP for Rn. Sibling encode_ldur_stur is LDUR/STUR/LDTR/STTR auto-size (different job).
- Dispatch: encoder/mod.rs:340-343 ldtrh/sttrh/ldtrb/sttrb => encode_ldtr_sized.
- Callers: encoder dispatch only (no codegen sites emit these mnemonics).

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- get_reg discards is_64, so Xt dest encodes as Wt (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP as Rt encodes as WZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- W base and XZR base encode as Xn/SP (see bugs).
- Out-of-range offsets wrap with imm9 = offset & 0x1FF (see bugs).
- Non-Mem forms correctly return Err.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ldtr_sized (arity / extra / Xt dest / SP / FP / W-base / XZR-base / offset range / pre/post/regoff / w31 alias / uppercase).
- Seven failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldtr_sized_*.md.

# Confirmed invariants (encode_ldrsw)

- Valid unsigned LDRSW Xt, [Xn|SP, #pimm] with pimm = imm12*4 in [0, 16380] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Alternate spellings x31, uppercase Xn/SP/XZR, lr match llvm-mc (1000 cases).
- Valid unscaled (simm9 in [-256,255]), pre-index, and post-index forms match llvm-mc when writeback does not use Rt==Rn (unless Rn is SP) (1000 cases).
- Valid register-offset LDRSW with lsl/sxtx on Xm and uxtw/sxtw on Wm, amount in {0,2}, matches llvm-mc (1000 cases).
- Success-path unsigned word: size=10 111 V=0 01 opc=10 imm12 Rn Rt. Equivalently w = 0xB9800000 | (imm12<<10) | (rn<<5) | rt.
- Unscaled: bits[25:24]=00, bit21=0, bits[11:10]=00, imm9 at [20:12]. Pre bits[11:10]=11; post=01. Register: bit21=1, bits[11:10]=10.
- Metamorphic: Rt+1 adds 1, Rn+1 adds 32, imm12+1 adds 1<<10; pre XOR post = 0b10<<10 (1000 cases).
- Fewer than 2 operands, Imm/Cond/Barrier/Shift/Extend/RegList/MemExpr/Label at the address slot, lsl #1/#3, uxtx, and base "foo" always Err (1000 cases).
- Known-answer: `ldrsw x0, [x1]` = 0xB9800020; `ldrsw x0, [x1, #4]` = 0xB9800420; `ldrsw x0, [x1, #16380]` = 0xB9BFFC20; `ldrsw x0, [x1, #-4]` = 0xB89FC020; `ldrsw x0, [x1, #4]!` = 0xB8804C20; `ldrsw x0, [x1], #4` = 0xB8804420; `ldrsw x0, [x1, x2]` = 0xB8A26820; `ldrsw x0, [sp, #4]` = 0xB98007E0; `ldrsw xzr, [x1]` = 0xB980003F; `ldrsw x30, [x2]` = 0xB980005E.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM LDRSW unsigned: size=10 111 0 01 10 imm12 Rn Rt, pimm in [0,16380] multiple of 4. LDURSW / pre / post: simm9 in [-256,255]. Register: option in {UXTW,LSL,SXTW,SXTX}, S amount 0 or 2. Literal: 10 011 000 imm19 Rt. Dest Xt (31=XZR), base Xn|SP.
- Dispatch: encoder/mod.rs:333 "ldrw" | "ldrsw" => encode_ldrsw.
- Callers: prologue.rs IrType::I32 => ldrsw; peephole.rs ldrsw forwarding; emit.rs; variadic.rs.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- get_reg discards is_64, so Wt dest encodes (see bugs).
- parse_reg_num maps sp to 31, so SP as Rt encodes as XZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- W base and XZR base encode as Xn/SP (see bugs).
- Bare W index defaults to LSL rather than requiring uxtw/sxtw (see bugs).
- Out-of-range offsets fall through to unscaled with imm9 = offset & 0x1FF (see bugs).
- Pre/post Rt==Rn is encoded (llvm-mc: unpredictable) (see bugs).
- Operand::Symbol (LDRSW literal) is not handled (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ldrsw (unsigned / unscaled / pre / post / regoff / alt-spellings / bad extend / invalid base / literal / extra / W-dest / SP / FP / W-base / XZR-base / W-index / writeback overlap / offset range).
- Ten failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldrsw_*.md.

# Confirmed invariants (encode_ldaxr_stlxr)

- Valid LDAXR/STLXR/LDAXRB/STLXRB/LDAXRH/STLXRH with Rt/Rn/Ws in 0..31 (xzr/wzr at 31 for data/status, sp at 31 for base, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Alternate spellings x31, uppercase Xn/SP/XZR, lr match llvm-mc (1000 cases).
- encode_ldaxr_stlxr XOR encode_ldxr_stxr at equal operands = 1<<15 (ARM ARM o0) (1000 cases). ldaxr XOR stlxr(wzr) = 1<<22 (L). X XOR W = 1<<30 (size). stlxrb XOR stlxrh = 1<<30.
- Success-path word: size 001000 0 L 0 Rs o0=1 Rt2=11111 Rn Rt. Equivalently load w = (size<<30) | 0x085FFC00 | (rn<<5) | rt; store w = (size<<30) | 0x0800FC00 | (ws<<16) | (rn<<5) | rt. size is 0b11/0b10 for X/W, 0b00 byte, 0b01 half.
- Fewer than required operands, Imm/Symbol/pre/post-index, invalid base names (foo, x32), MemRegOffset, and MemExpr always Err (1000 cases).
- Known-answer: `ldaxr x0, [x1]` = 0xC85FFC20; `ldaxr w0, [x1]` = 0x885FFC20; `ldaxrb w0, [x1]` = 0x085FFC20; `ldaxrh w0, [x1]` = 0x485FFC20; `stlxr w0, x1, [x2]` = 0xC800FC41; `stlxr w0, w1, [x2]` = 0x8800FC41; `ldaxr x0, [sp]` = 0xC85FFFE0; `ldaxr lr, [x2]` = 0xC85FFC5E.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Load/Store Exclusive LDAXR/STLXR: size 001000 0 L 0 Rs o0=1 Rt2=11111 Rn Rt. Syntax LDAXR Wt/Xt, [Xn|SP]{,#0}; STLXR Ws, Wt/Xt, [Xn|SP]{,#0}; byte/half take Wt. Register 31 is ZR for Rt/Ws, SP for Rn. Sibling encode_ldxr_stxr is o0=0 (different job).
- Dispatch: encoder/mod.rs:354-359 ldaxr/stlxr/ldaxrb/stlxrb/ldaxrh/stlxrh => encode_ldaxr_stlxr.
- Callers: src/backend/arm/codegen/inline_asm.rs Acquire => ldaxr, Release => stlxr, AcqRel/SeqCst => both.

## Quirks

- Extra operands beyond the exclusive arity are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP as Rt encodes as ZR (see bugs).
- W register as base and XZR as base encode as Xn/SP (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- get_reg discards is_64 for STLXR status, so X-as-Ws encodes (see bugs).
- forced_size overrides data width, so ldaxrb Xt encodes as Wt (see bugs).
- Mem { base, .. } ignores offset, so nonzero exclusive offset encodes as [Xn] (see bugs).
- No Ws-vs-Rt/Rn overlap check (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ldaxr_stlxr (arity / extra / SP / FP / W-base / XZR-base / X-Ws / offset / Ws-overlap / X-data-byte / alt-spellings / MemRegOffset).
- Nine failing negative-contract properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldaxr_stlxr_*.md.

# Confirmed invariants (encode_uxtw)

- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty, x-1, x99, w), and non-register kinds at GPR slots always Err (1000 cases each).
- Known-answer (llvm-mc, not SUT): `uxtw x0, w1` = 0xD3407C20; `uxtw xzr, wzr` = 0xD3407FFF; `uxtw lr, w0` = 0xD3407C1E; `ubfm x0, x1, #0, #31` aliases to 0xD3407C20. SUT currently emits 32-bit ORR/MOV instead (see bugs).
- Intended success-path word (ARM ARM / llvm-mc): sf=1 opc=10 bits[28:23]=100110 N=1 immr=0 imms=31 Rn Rd. Equivalently w = 0xD3407C00 | (rn<<5) | rd. SUT does not satisfy this (MOV encoding).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM C6 UXTW: alias of UBFM Xd, Xn, #0, #31. sf=1 opc=10 N=1 immr=0 imms=31 Rn Rd. Syntax UXTW Xd, Wn only. Register 31 is XZR/WZR, never SP. Sibling encode_sxtw is SBFM (opc=00, different job). Sibling encode_uxth/uxtb use imms=15/7.
- Dispatch: encoder/mod.rs:296 `"uxtw" => encode_uxtw(operands)` (scalar only; no NEON arrangement path).
- Callers: assembler README Extensions table lists uxtw.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- is_64 from get_reg is discarded; W dest is encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Body emits 32-bit ORR (MOV Wd, Wn) instead of 64-bit UBFM (see bugs). The producing comment mentions both encodings; ARM ARM / llvm-mc / gas require UBFM.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_uxtw (arity / extra / Wd / SP / FP / nonreg / invalid name / alt-spellings / UBFM alias / ARM fields).
- Five failing properties (plus KATs/regressions) are SUT bugs, not quirks: MOV-not-UBFM, extra operand, W dest, SP-as-ZR, FP-as-GPR. See pbt-out/bug_reports/encode_uxtw_*.md.

# Confirmed invariants (encode_umull)

- Valid UMULL Xd, Wn, Wm with Rd/Rn/Rm in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR, LR, uppercase match llvm-mc (1000 cases).
- encode_umull XOR encode_smull at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- encode_umull(Xd,Wn,Wm) = encode_umaddl(Xd,Wn,Wm,XZR) = llvm-mc of both mnemonics (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 101, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9BA07C00 | (rm<<16) | (rn<<5) | rd.
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `umull x0, w1, w2` = 0x9ba27c20; `umull xzr, wzr, wzr` = 0x9bbf7fff; `umull lr, w1, w2` = 0x9ba27c3e; `umaddl x0, w1, w2, xzr` aliases to 0x9ba27c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) UMULL: alias of UMADDL with Ra=XZR. sf=1 op54=00 11011 U=1 01 Rm o0=0 Ra=11111 Rn Rd. Syntax UMULL Xd, Wn, Wm. Register 31 is XZR/WZR, never SP. Sibling encode_smull is U=0 (different job).
- Dispatch: encoder/mod.rs:258-267 `"umull"` + first operand RegArrangement => NEON path, else scalar encode_umull.
- Callers: assembler README Data Processing table lists umull.

## Quirks

- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_umull (arity / extra / width / SP / FP / nonreg / invalid name / alt-spellings / U bit / alias / ARM fields).
- Four failing negative-contract properties are SUT bugs, not quirks: extra operand, wrong width, SP-as-ZR, FP-as-GPR. See pbt-out/bug_reports/encode_umull_*.md.

# Confirmed invariants (encode_neon_rbit)

- Valid RBIT Vd.T, Vn.T with T in {8b,16b} and Vd/Vn in v0..v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Q bit: encode(.8b) XOR encode(.16b) = 1<<30 (1000 cases).
- Success-path word: 0 Q 1 01110 01 10000 00101 10 Rn Rd. Equivalently w = (Q<<30) | 0x2E605800 | (rn<<5) | rd.
- Rd+1 adds 1; Rn+1 adds 32 (1000 cases).
- Fewer than 2 operands, dest T not in {8b,16b}, Imm/Mem/Shift/RegList/Label dest, Imm source, and invalid names (v32, foo, empty, v, v99, v-1) always Err.
- Known-answer: `rbit v0.8b, v1.8b` = 0x2e605820; `rbit v31.16b, v0.16b` = 0x6e60581f; `rbit v31.8b, v31.8b` = 0x2e605bff.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD two-register miscellaneous RBIT (vector): T is 8B or 16B. Encoding 0 Q 1 01110 01 10000 00101 10 Rn Rd.
- Dispatch: encoder/mod.rs:902-909 `"rbit"` + first operand RegArrangement => encode_neon_rbit, else scalar encode_rbit.
- Parser lowercases arrangements; `is_register` accepts x/w/d/s/q/v/h/b and sp/wsp/xzr/wzr/lr, so `x0.8b` and `sp.8b` are caller-reachable.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Operand::Reg source is accepted (see bugs).
- parse_reg_num accepts x/w/d/s/q/h/b prefixes and maps sp to 31 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_neon_rbit (arity / extra / T / Q / Rd / Rn / mismatch / bare src / Imm / invalid name / prefix / SP).

# Confirmed invariants (encode_umulh)

- Valid UMULH Xd, Xn, Xm with Rd/Rn/Rm in 0..31 (xzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31, XZR, LR, uppercase Xn match llvm-mc (1000 cases).
- encode_umulh XOR encode_smulh at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 110, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9BC07C00 | (rm<<16) | (rn<<5) | rd.
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `umulh x0, x1, x2` = 0x9bc27c20; `umulh xzr, xzr, xzr` = 0x9bdf7fff; `umulh lr, x1, x30` = 0x9bde7c3e; `umulh x0, x1, xzr` = 0x9bdf7c20; `smulh x0, x1, x2` = 0x9b427c20 (XOR = 1<<23).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) UMULH: sf=1 op54=00 11011 op31=110 Rm o0=0 Ra=11111 Rn Rd. Syntax UMULH Xd, Xn, Xm. No 32-bit form. Register 31 is XZR, never SP. Sibling encode_smulh is U=0 (different job).
- Dispatch: encoder/mod.rs:274 `"umulh" => encode_umulh(operands)` (scalar only; no NEON arrangement path).
- Callers: assembler README Data Processing table lists umulh.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- is_64 from get_reg is discarded; W registers are encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr).

---

# Confirmed invariants (encode_umaddl)

- Valid UMADDL Xd, Wn, Wm, Xa with Rd/Rn/Rm/Ra in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR/WZR, LR, uppercase Xn/Wn match llvm-mc (1000 cases).
- encode_umaddl(Xd, Wn, Wm, XZR) equals encode_umull(Xd, Wn, Wm), and both match llvm-mc `umaddl ..., xzr` / `umull` (1000 cases).
- encode_umaddl XOR encode_smaddl at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 101, Rm at [20:16], o0=0 at 15, Ra at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9BA00000 | (rm<<16) | (ra<<10) | (rn<<5) | rd.
- Fewer than 4 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `umaddl x0, w1, w2, x3` = 0x9ba20c20; `umaddl xzr, wzr, wzr, xzr` = 0x9bbf7fff; `umaddl lr, w1, w2, x30` = 0x9ba2783e; `umaddl x0, w1, w2, xzr` / `umull x0, w1, w2` aliases to 0x9ba27c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) UMADDL: sf=1 U=1 11011 101 Rm o0=0 Ra Rn Rd. UMULL Xd, Wn, Wm is the alias of UMADDL Xd, Wn, Wm, XZR. Register 31 is XZR/WZR, never SP/WSP. Dest and accumulator are Xd/Xa; multiply sources are Wn/Wm.
- Dispatch: encoder/mod.rs:270 `"umaddl" => encode_umaddl(operands)` (scalar only; no NEON arrangement path).
- Sibling encode_umull is the same format with Ra=XZR (alias). Sibling encode_smaddl is U=0 (different job).
- Callers: assembler README Data Processing table lists umaddl. No codegen emission of scalar umaddl found.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- is_64 from get_reg is discarded; W dest, X sources, and W acc are encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr).

---

# Confirmed invariants (encode_sxtw)

- Valid SXTW Xd, Wn with Rd/Rn in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR, LR, uppercase, and Xd,Xn (llvm-mc canonicalizes to Xd,Wn) match llvm-mc (1000 cases).
- encode_sxtw(Xd, Wn) equals encode_sbfm(Xd, Xn, #0, #31), and both match llvm-mc `sxtw` (1000 cases).
- Success-path word: sf=1 at bit 31, opc=00 at [30:29], 100110 at [28:23], N=1 at 22, immr=0 at [21:16], imms=31 at [15:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x93407C00 | (rn<<5) | rd.
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `sxtw x0, w1` = 0x93407c20; `sxtw xzr, wzr` = 0x93407fff; `sxtw lr, w0` = 0x93407c1e; `sbfm x0, x1, #0, #31` aliases to the same word as `sxtw x0, w1`.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM SXTW is the alias of SBFM Xd, Xn, #0, #31: sf=1 00 100110 N=1 immr=0 imms=31 Rn Rd. Assembler syntax: SXTW Xd, Wn only (no Wd form). Register 31 is XZR/WZR, never SP/WSP. llvm-mc also accepts SXTW Xd, Xn (canonicalizes source to W).
- Dispatch: encoder/mod.rs:293 `"sxtw" => encode_sxtw(operands)`.
- Sibling encode_sbfm is the same format with caller immr/imms (alias at #0,#31). Sibling encode_sxth is imms=15 (different job). Sibling encode_uxtw is UBFM/MOV (different job).
- Callers: assembler README Extensions table lists sxtw. Codegen emits `sxtw x0, w0` in cast_ops.rs / atomics.rs / f128.rs / alu.rs / peephole.rs.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- Dest/src width from get_reg is discarded; W dest is encoded as 64-bit SXTW (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / W dest / SP / FP / non-Reg / invalid name / x31 / uppercase / lr / Xd,Xn).

# Confirmed invariants (encode_sxth)

- Valid SXTH Wd, Wn and Xd, Wn with Rd/Rn in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR, LR, uppercase, and Xd,Xn (llvm-mc canonicalizes to Xd,Wn) match llvm-mc (1000 cases).
- encode_sxth(Rd, Rn) equals encode_sbfm(Rd, Rn, #0, #15) with matching dest width, and both match llvm-mc `sxth` (1000 cases).
- Success-path word: sf at bit 31, opc=00 at [30:29], 100110 at [28:23], N=sf at 22, immr=0 at [21:16], imms=15 at [15:10], Rn at [9:5], Rd at [4:0]. Equivalently w = (is_64 ? 0x93403C00 : 0x13003C00) | (rn<<5) | rd.
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `sxth w0, w1` = 0x13003c20; `sxth x0, w1` = 0x93403c20; `sxth wzr, wzr` = 0x13003fff; `sbfm w0, w1, #0, #15` aliases to the same word as `sxth w0, w1`.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM SXTH is the alias of SBFM Rd, Rn, #0, #15: sf 00 100110 N=sf immr=0 imms=15 Rn Rd. Assembler syntax: SXTH Wd, Wn or SXTH Xd, Wn. Register 31 is WZR/XZR, never SP/WSP. llvm-mc also accepts SXTH Xd, Xn (canonicalizes source to W).
- Dispatch: encoder/mod.rs:294 `"sxth" => encode_sxth(operands)`.
- Sibling encode_sbfm is the same format with caller immr/imms (alias at #0,#15). Sibling encode_sxtb is imms=7 (different job). Sibling encode_uxth is UBFM opc=10 (different job).
- Callers: assembler README Extensions table lists sxth. Codegen emits `sxth x0, w0` in cast_ops.rs / atomics.rs / f128.rs.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- is_64 from source get_reg is discarded; W dest + X source is encoded as 32-bit SXTH (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr / Xd,Xn).

---

# Confirmed invariants (encode_smull)

- Valid SMULL Xd, Wn, Wm with Rd/Rn/Rm in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR/WZR, LR, uppercase Xn/Wn match llvm-mc (1000 cases).
- encode_smull(Xd, Wn, Wm) equals encode_smaddl(Xd, Wn, Wm, XZR) and both match llvm-mc `smull` / `smaddl ..., xzr` (1000 cases).
- encode_smull XOR encode_umull at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 001, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9B207C00 | (rm<<16) | (rn<<5) | rd.
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `smull x0, w1, w2` = 0x9b227c20; `smull xzr, wzr, wzr` = 0x9b3f7fff; `smull lr, w1, w2` = 0x9b227c3e; `smaddl x0, w1, w2, xzr` aliases to the same word.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) SMADDL: sf=1 U=0 11011 001 Rm o0=0 Ra Rn Rd. SMULL Xd, Wn, Wm is the alias of SMADDL Xd, Wn, Wm, XZR. Register 31 is XZR/WZR, never SP/WSP. Dest is Xd; sources are Wn/Wm.
- Dispatch: encoder/mod.rs:247-256 `"smull"` with RegArrangement goes to NEON; otherwise encode_smull (scalar). This campaign tests only the scalar helper.
- Sibling encode_smaddl is the same format with caller Ra (alias at Ra=XZR). Sibling encode_umull is U=1 (different job).
- Callers: assembler README Data Processing table lists smull. No codegen emission of scalar smull found.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- is_64 from get_reg is discarded; W dest and X sources are encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr).

---

# Confirmed invariants (encode_shift)

- Valid GP dest matching mnemonic size b/w/l/q, Imm count 0..255 matches llvm-mc `-triple=x86_64 -show-encoding` (1000 cases).
- Valid %cl count form matches llvm-mc (1000 cases).
- 1-operand form matches llvm-mc omitted-count encoding (1000 cases).
- 1-operand encoding equals Imm(1) two-operand encoding (GAS omitted count is 1) (1000 cases).
- Changing only Group 2 /digit (ROL/ROR/RCL/RCR/SHL/SHR/SAR) differs only in ModR/M bits [5:3] (1000 cases).
- Memory dest without segment, including (%rsp)/(%r12) SIB and (%rbp)/(%r13) disp8, matches llvm-mc (1000 cases).
- RIP-relative memory with trailing imm8 (count 2..255) produces one R_X86_64_PC32 reloc with addend -5 (1000 cases).
- Arity 0/3/4 and non-CL register count always Err (1000 cases).
- 1-operand Imm/Label/Indirect always Err (1000 cases).
- Known-answer: `shlq $1, %rax` / `shlq %rax` = [0x48,0xd1,0xe0]; `shll $1, %eax` = [0xd1,0xe0]; `shlw $1, %ax` = [0x66,0xd1,0xe0]; `shlb $1, %al` = [0xd0,0xe0]; `shlq %cl, %rax` = [0x48,0xd3,0xe0]; `shlq $2, %rax` = [0x48,0xc1,0xe0,0x02].

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=x86_64 -show-encoding
- Intel SDM Group 2: D0/D2/C0 r/m8; D1/D3/C1 r/m16/32/64; /0 ROL /1 ROR /2 RCL /3 RCR /4 SHL/SAL /5 SHR /7 SAR. 66 prefix for 16-bit; REX.W for 64-bit. Count is 1, CL, or imm8. GAS omitted count is 1. AT&T operand order is count, dest. SAL is alias of SHL.
- Dispatch: encoder/mod.rs:196-200,670-671 suffixed forms; suffix-less shl/sal/shr/sar/rol/ror/rcl/rcr go through encode_suffixless_shift then encode_shift.
- Callers: assembler README Shifts/Rotates table; codegen/emit.rs:151-153 emits shll/shlq, sarl/sarq, shrl/shrq.
- Siblings encode_double_shift (SHLD/SHRD), encode_sse_shift, encode_avx_shift, encode_bmi2_shift are different jobs.

## Quirks

- FS/GS segment override is not emitted (see bugs).
- Size-mismatched and non-GP dest registers are encoded via reg_num aliases (see bugs).
- Imm count is truncated with `as u8` so 256 encodes as 0 (see bugs).
- llvm-mc accepts `$ -1` as 255; SUT does the same via `as u8` (agreement, not a bug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (1-op Reg/Mem/other, arity, Imm+Reg, Imm+Mem count==1 vs else, CL+Reg, CL+Mem, RIP reloc addend, extra/non-CL, mixed size, non-GP, segment).

---

# Confirmed invariants (encode_sbc)

- Valid three-GPR same-width SBC/SBCS with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_sbc XOR encode_sbc(set_flags=true) = 1<<29 (ARM ARM S bit) (1000 cases).
- encode_sbc XOR encode_adc at equal operands = 1<<30 (ARM ARM op SBC=1 vs ADC=0) (1000 cases).
- encode_sbc(Rd, ZR, Rm, set_flags) equals llvm-mc `ngc`/`ngcs` Rd, Rm (ARM ARM NGC alias) (1000 cases).
- `lr` in any slot encodes as X30 and matches llvm-mc (1000 cases).
- Success-path word: sf at 31, op=1 at 30, S at 29, bits [28:21]=11010000, Rm at [20:16], bits [15:10]=000000, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `sbc x0, x1, x2` encodes as 0xda020020; `sbcs w0, w1, w2` as 0x7a020020; `ngc x0, x1` / `sbc x0, xzr, x1` as 0xda0103e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Add/subtract (with carry) SBC: `sf 1 S 11010000 Rm 000000 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd/Rn/Rm same width. No shifted-register form. NGC Rd, Rm is alias of SBC Rd, ZR, Rm.
- `lr` is a 64-bit alias of X30 (llvm-mc and parse_reg_num).
- Dispatch: encoder/mod.rs:283-284 `"sbc" => encode_sbc(operands, false)`, `"sbcs" => encode_sbc(operands, true)`. Sibling encode_adc is ADC (op=0), different job.
- Callers: assembler README data-processing table lists sbc/sbcs; codegen/i128_ops.rs:73 emits `sbc x1, x3, x5`.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..2 / sf / S / extra / mixed / SP / FP / lr / invalid name / non-Reg).

---

# Confirmed invariants (encode_ret)

- Valid RET with omitted Rn or Rn in {x0–x30, xzr, lr} (including uppercase X0) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Bare `ret` ≡ `ret x30` ≡ `ret lr` = Word(0xd65f03c0) (ARM ARM omitted Xn is X30).
- encode_ret XOR encode_br at equal Rn = 1<<22 (ARM ARM opc RET=0010 vs BR=0000) (1000 cases).
- Success-path word: bits[31:25]=1101011, opc[24:21]=0010, op2[20:16]=11111, op3[15:10]=000000, Rn[9:5], op4[4:0]=00000; w = 0xd65f0000 | (rn << 5).
- Non-register operand kinds (Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label) always Err.
- Invalid register names (x32, w32, foo, empty, r0, x, x-1, x99) always Err.
- Known-answer: `ret` encodes as 0xd65f03c0; `ret x0` as 0xd65f0000; `ret xzr` as 0xd65f03e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Unconditional branch (register) RET: bits[31:25]=1101011 opc=0010 op2=11111 op3=000000 Rn[9:5] op4=00000. Omitted Xn defaults to X30. Rn is Xn; register 31 is XZR, never SP.
- Dispatch: encoder/mod.rs:320 `"ret" => encode_ret`. Sibling encode_br is BR (opc=0000), different job. Sibling encode_blr is BLR (opc=0001), different job.
- Callers: assembler README Branches table lists ret; codegen/prologue.rs:319 emits bare `ret`.

## Quirks

- Extra operands beyond index 0 are ignored (see bugs).
- W-form Rn is accepted and encoded as the matching X register (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as XZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- llvm-mc accepts `ret x31` as `ret xzr`; SUT parse_reg_num also maps x31 to 31 (agreement, not a bug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (empty-default / get_reg success / get_reg None / get_reg other / extra / W / SP / FP).

---

# Confirmed invariants (encode_orn)

- Valid three-GPR same-width ORN with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR/ROR in range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid vector ORN with T in {8b,16b}, Vd/Vn/Vm in v0–v31 matches llvm-mc (1000 cases).
- encode_orn XOR encode_logical(opc=01) = 1<<21 (ARM ARM N bit vs ORR) (1000 cases).
- encode_orn(Rd, ZR, Rm, shift) equals encode_mvn(Rd, Rm, shift) and llvm-mc `orn Rd, ZR, Rm` (documented MVN alias) (1000 cases).
- encode_orn(X-ops) XOR encode_orn(W-ops) at equal register numbers and amt in 0..31 = 1<<31 (ARM ARM sf) (1000 cases).
- Vector T=8b XOR T=16b at equal Rd/Rn/Rm = 1<<30 (ARM ARM Q) (1000 cases).
- Success-path GPR word: sf at 31, opc=01 at [30:29], bits [28:24]=01010, shift at [23:22], N=1 at 21, Rm at [20:16], imm6 at [15:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `orn x0, x1, x2` encodes as 0xaa220020; `orn w0, w1, w2` as 0x2a220020; `orn v0.8b, v1.8b, v2.8b` as 0x0ee21c20; `orn v0.16b, v1.16b, v2.16b` as 0x4ee21c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Logical (shifted register) ORN: `sf 01 01010 shift N=1 Rm imm6 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd/Rn/Rm same width. shift in {LSL,LSR,ASR,ROR}. imm6 0..31 (sf=0) or 0..63 (sf=1).
- ARM ARM Advanced SIMD three-same ORN: `0 Q 0 01110 11 1 Rm 00011 1 Rn Rd`. T in {8B,16B}. Q=1 iff T=16B.
- GNU as / llvm-mc alias: `orn Rd, Rn, #imm` encodes as `orr Rd, Rn, #~imm`.
- Documented MVN alias at data_processing.rs:753: MVN Rd, Rm -> ORN Rd, XZR, Rm.
- Dispatch: encoder/mod.rs:235 `"orn" => encode_orn`. Sibling encode_eon is EON (opc=10), different job. Sibling encode_logical(opc=01) is ORR (N=0), different job.
- Callers: assembler README data-processing and NEON three-same tables list orn.

## Quirks

- Immediate form is not implemented (see bugs).
- Extra operands beyond the optional shift are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Unknown shift kind defaults to LSL (see bugs).
- Shift amount is masked with 0x3F; 32-bit amounts 32..63 encode UNALLOCATED imm6<5>=1 (see bugs).
- NEON T other than 16b encodes Q=0, including 8h/4h/4s/2s/2d/1d (see bugs).
- Source NEON arrangements are discarded; get_neon_reg accepts Operand::Reg (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Imm / Shift / NEON T / extra / get_reg kinds / Q / sf).

---

# Confirmed invariants (encode_neon_tbl)

- Valid vector TBL with Ta in {8b,16b}, Vd/Vm in v0–v31, 1–4 consecutive wrapping table registers all .16B matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Ta=8b XOR Ta=16b at equal Rd/Rn/Rm/len = 1<<30 (1000 cases).
- Changing only nregs in {1,2,3,4} differs only in len bits [14:13]; len = nregs-1 (1000 cases).
- Success-path word: bit 31=0, Q at 30, bits [29:24]=001110, bits [23:21]=000, Rm at [20:16], bit 15=0, len at [14:13], op=0 at 12, bits [11:10]=00, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, missing RegList, and invalid dest names always Err.
- Known-answer: `tbl v0.8b, {v1.16b}, v2.8b` encodes as 0x0e020020; `tbl v0.16b, {v1.16b}, v2.16b` as 0x4e020020; 2-reg 0x0e032020; 3-reg 0x4e044020; 4-reg 0x0e056020; wrap `{v31.16b, v0.16b}` as 0x0e0223e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD table lookup TBL: `0 Q 00 1110 00 0 Rm 0 len op 00 Rn Rd` with op=0. Ta in {8B,16B}. Table is 1–4 consecutive .16B registers wrapping at 31. Vm.Ta matches Vd.Ta. Q=1 iff Ta=16B.
- Dispatch: encoder/mod.rs:729 `"tbl" => encode_neon_tbl`. Sibling encode_neon_tbx is TBX (op=1), different job.
- Callers: assembler README NEON permute table lists tbl/tbx.
- Parser `parser.rs:2030-2072` builds Operand::RegList; rejects empty lists; range syntax expands wrapping consecutives. Encoder still panics if given an empty list directly.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Ta other than 16b encodes Q=0, including 4h/8h/2s/4s/2d/1d (see bugs).
- Empty RegList panics on regs[0] (see bugs).
- nregs>4 wraps via `(num_regs-1)&0x3` (see bugs).
- Only first list register number and len are encoded; later names/arrangements and sequentiality are ignored (see bugs).
- get_neon_reg accepts Operand::Reg, so GPR dest/Vm and bare V in the list encode (see bugs).
- Vm arrangement is discarded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Ta / table list / get_neon_reg Reg dest+Vm / extra / mismatched T).

---

# Confirmed invariants (encode_neon_shift_imm)

- Valid vector USHR with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [1, esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-esize Q=0 vs Q=1 arrangements XOR = 1<<30 (1000 cases).
- Success-path word: bit 31=0, Q at 30, U=1 at 29, bits [28:23]=011110, immh:immb at [22:16]=2*esize-shift, opcode at [15:10]=000001, Rn at [9:5], Rd at [4:0].
- T=1d (Reserved Q=0 && esize==64), fewer than 3 operands, GPR/FP dest, and invalid dest names always Err.
- Known-answer: `ushr v0.8b, v1.8b, #1` encodes as 0x2f0f0420; `ushr v0.16b, v1.16b, #8` as 0x6f080420; `ushr v0.4h, v1.4h, #1` as 0x2f1f0420; `ushr v0.8h, v1.8h, #16` as 0x6f100420; `ushr v0.2s, v1.2s, #1` as 0x2f3f0420; `ushr v0.4s, v1.4s, #32` as 0x6f200420; `ushr v0.2d, v1.2d, #1` as 0x6f7f0420; `#64` as 0x6f400420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate USHR: `0 Q 1 011110 immh immb 00000 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 is Reserved. shift = (2*esize)-UInt(immh:immb) in [1, esize].
- Dispatch: encoder/mod.rs:658-659 uses encode_neon_ushr / encode_neon_sshr. This symbol is a dead `pub(crate)` helper (`#![allow(dead_code)]`). Documented job remains USHR (neon.rs:372).
- Callers: none. Assembler README NEON shifts table lists ushr/sshr.

## Quirks

- `_is_unsigned` is unused (Rust `_` prefix); U is hardcoded to 1. Docstring says USHR.
- Source arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- Negative Imm panics in debug; shift 0 / esize+1 wrap/mask (see bugs).
- Shift is `get_imm` then `as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- get_neon_reg accepts Operand::Reg, so a bare GPR/FP/V source encodes as Rn (see bugs). Dest as Operand::Reg still Errs via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / T / get_imm as u32 / get_neon_reg Reg dest+source / extra / mismatched T).

---

# Confirmed invariants (encode_negs)

- Valid two-GPR same-width NEGS with Rd/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR in range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_negs(Rd, Rm, shift) equals llvm-mc `negs Rd, Rm, shift` and llvm-mc `subs Rd, ZR, Rm, shift` (1000 cases). Documented alias at data_processing.rs:728.
- encode_negs(X-ops) XOR encode_negs(W-ops) at equal register numbers and amt in 0..31 = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, op=1 at 30, S=1 at 29, bits [28:24]=01011, shift at [23:22], bit 21=0, Rm at [20:16], imm6 at [15:10], Rn=31 at [9:5], Rd at [4:0].
- `lr` in either slot encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at Rd/Rm always Err.
- Known-answer: `negs x0, x1` / `subs x0, xzr, x1` encode as 0xeb0103e0; `negs w0, w1` as 0x6b0103e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Add/subtract (shifted register) NEGS (alias of SUBS): `sf 1 1 01011 shift 0 Rm imm6 11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd and Rm same width. Exactly two registers plus optional shift. shift in {LSL,LSR,ASR} (not ROR). imm6 0..31 (sf=0) or 0..63 (sf=1).
- `lr` is a 64-bit alias of X30 (llvm-mc and parse_reg_num).
- Dispatch: encoder/mod.rs:279 `"negs" => encode_negs`. Sibling encode_neg is SUB (S=0), different job.
- Callers: assembler README data-processing table; no codegen emission of `negs` found.

## Quirks

- Extra operands beyond the optional shift are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Unknown shift kind (including ROR) defaults to LSL (see bugs).
- Shift amount is masked with 0x3F; 32-bit amounts 32..63 encode UNALLOCATED imm6<5>=1 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..1 / sf / Shift at 2 / shift-kind / imm6 / Rn=31 / invalid name / non-Reg).

---

# Confirmed invariants (encode_neg)

- Valid two-GPR NEG with ABI names / x0–x31 / fp matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases).
- encode_neg(rd, rs) equals llvm-mc `sub rd, x0, rs` (1000 cases). Documented expansion at README.md:321.
- encode_neg(rd, rs) equals encode_alu_reg([rd, x0, rs], funct3=000, funct7=0100000) (1000 cases).
- Success-path word: opcode[6:0]=0110011, rd[11:7], funct3[14:12]=000, rs1[19:15]=0, rs2[24:20], funct7[31:25]=0100000.
- ABI names and xN (and fp/s0) of the same number encode identically (1000 cases).
- Imm(n) for n in 0..=31 encodes as register xN (get_reg GCC inline-asm extension) (1000 cases).
- Fewer than 2 operands, FP/vector/unknown names, out-of-range Imm, and non-Reg kinds always Err.
- Known-answer: `neg a0, a1` / `sub a0, x0, a1` encode as 0x40b00533; `neg zero, zero` as 0x40000033; `neg t6, ra` / `neg x31, x1` as 0x40100fb3; `neg fp, s0` / `neg x8, x8` as 0x40800433.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding
- RISC-V Unprivileged ISA: NEG rd, rs = SUB rd, x0, rs. SUB R-type opcode OP=0110011, funct3=000, funct7=0100000, rs1=x0.
- Dispatch: encoder/mod.rs:749 `"neg" => encode_neg`. Sibling encode_negw is SUBW (out of scope).
- Callers: alu.rs:23 `neg t0, t0`; atomics.rs:450 `neg t2, t2`; intrinsics.rs `neg t3/t5`.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- get_reg accepts Imm 0..=31 as a register number (Doc evidence: encoder/mod.rs:351-352 GCC inline asm).
- reg_num case-folds; llvm-mc rejects uppercase ABI names. SUT is more lenient on codegen-emitted lowercase assembly.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg Reg/Imm/other/missing, extra operand, SUB expansion vs llvm-mc).

---

# Confirmed invariants (encode_neon_shift_right)

- Valid vector SRSHR/URSHR/SSRA/USRA/SRSRA/URSRA with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [1, esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29 (ARM ARM U bit) (1000 cases).
- Same-esize Q=0 vs Q=1 arrangements XOR = 1<<30; opcode pairs differ only in bits [15:10] (1000 cases).
- Success-path word: bit 31=0, Q at 30, U at 29, bits [28:23]=011110, immh:immb at [22:16]=2*esize-shift, opcode at [15:10], Rn at [9:5], Rd at [4:0].
- Shift 0, esize+1, negative, and 1d (Reserved Q=0 && esize==64) always Err; fewer than 3 operands, GPR/FP dest, invalid names, and non-matching operand kinds always Err.
- Known-answer: `srshr v0.8b, v1.8b, #1` encodes as 0x0f0f2420; `urshr v0.16b, v1.16b, #8` as 0x6f082420; `ssra v0.4h, v1.4h, #1` as 0x0f1f1420; `usra v0.8h, v1.8h, #16` as 0x6f101420; `srsra v0.2s, v1.2s, #1` as 0x0f3f3420; `ursra v0.4s, v1.4s, #32` as 0x6f203420; `srshr v0.2d, v1.2d, #1` as 0x4f7f2420; `#64` as 0x4f402420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SRSHR/URSHR/SSRA/USRA/SRSRA/URSRA: `0 Q U 011110 immh immb opcode Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 is Reserved. shift = (2*esize)-UInt(immh:immb) in [1, esize]. opcode 001001 / 000101 / 001101. U=0 signed / U=1 unsigned.
- Dispatch: encoder/mod.rs:607-612. SSHR/USHR use encode_neon_sshr/ushr (out of scope).
- Callers: assembler README NEON shifts table; no codegen emission found.

## Quirks

- Source arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- Shift is `get_imm as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- get_neon_reg accepts Operand::Reg, so a bare GPR/FP/V source encodes as Rn (see bugs). Dest as Operand::Reg still Errs via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / T / get_imm as u32 / get_neon_reg Reg dest+source / extra / mismatched T).

---

# Confirmed invariants (encode_mvn)

- Valid two-GPR same-width MVN with Rd/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR/ROR in range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_mvn(Rd, Rm, shift) equals llvm-mc `mvn Rd, Rm, shift` and llvm-mc `orn Rd, ZR, Rm, shift` (1000 cases). Documented alias at data_processing.rs:753.
- encode_mvn(X-ops) XOR encode_mvn(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, opc=01 at [30:29], bits [28:24]=01010, shift at [23:22], N=1 at 21, Rm at [20:16], imm6 at [15:10], Rn=31 at [9:5], Rd at [4:0].
- Valid NEON MVN with T in {8b,16b}, Vd/Vn in v0–v31, matches llvm-mc `mvn` and llvm-mc `not` (1000 cases).
- `lr` in either scalar slot encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands always Err.
- Known-answer: `mvn x0, x1` encodes as 0xaa2103e0; `mvn w0, w1` as 0x2a2103e0; `orn x0, xzr, x1` as 0xaa2103e0; `mvn v0.16b, v1.16b` / `not v0.16b, v1.16b` as 0x6e205820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Logical (shifted register) MVN (alias of ORN): `sf 01 01010 shift 1 Rm imm6 11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd and Rm same width. Exactly two registers plus optional shift. shift in {LSL,LSR,ASR,ROR}. imm6 0..31 (sf=0) or 0..63 (sf=1).
- ARM ARM Advanced SIMD NOT (vector, alias MVN): `0 Q 1 01110 00 10000 00101 10 Rn Rd`. T in {8B,16B} only.
- `lr` is a 64-bit alias of X30 (llvm-mc and parse_reg_num).
- Dispatch: encoder/mod.rs:280 `"mvn" => encode_mvn`. NEON dest is branched inside encode_mvn to encode_neon_not.
- Callers: alu.rs:26 `mvn x0, x0`; i128_ops.rs:43-51 `mvn x0, x0` / `mvn x1, x1`; inline_asm.rs:354 `mvn dest, dest`.

## Quirks

- Extra operands beyond the optional shift are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Unknown shift kind defaults to LSL (see bugs).
- Shift amount is masked with 0x3F; 32-bit amounts 32..63 encode UNALLOCATED imm6<5>=1 (see bugs).
- encode_neon_not sets Q from dest=="16b" only; T not in {8b,16b} and mismatched source T are accepted (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..1 / sf / Shift at 2 / shift-kind / imm6 / Rn=31 / neon Q / neon extra).

---

# Confirmed invariants (encode_mul)

- Valid three-GPR same-width MUL with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_mul(Rd, Rn, Rm) equals llvm-mc `mul Rd, Rn, Rm` and llvm-mc `madd Rd, Rn, Rm, ZR` (1000 cases). Documented alias at data_processing.rs:589.
- encode_mul(X-ops) XOR encode_mul(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:21]=0011011000, Rm at [20:16], o0=0 at 15, Ra=31 at [14:10], Rn at [9:5], Rd at [4:0].
- Valid NEON MUL with T in {8b,16b,4h,8h,2s,4s}, Vd/Vn/Vm in v0–v31, matches llvm-mc (1000 cases).
- `lr` in any of the three scalar slots encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 3 operands, non-register operands (Imm/Symbol/Mem/Shift/Cond/Label), and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `mul x0, x1, x2` encodes as 0x9b027c20; `mul w0, w1, w2` as 0x1b027c20; `madd x0, x1, x2, xzr` as 0x9b027c20; `mul v0.16b, v1.16b, v2.16b` as 0x4e229c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) MUL (alias of MADD): `sf 00 11011 000 Rm 0 11111 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All three registers same width. Exactly three operands. o0 (bit 15) is 0. Ra (bits 14:10) is 31.
- ARM ARM Advanced SIMD MUL (vector): `0 Q 0 01110 size 1 Rm 10011 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S}. size==11 is UNDEFINED.
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:238-244 mul. NEON dest is routed to encode_neon_three_same / encode_neon_elem; scalar dest to encode_mul. encode_mul itself still has a RegArrangement branch to encode_neon_mul.
- Callers: alu.rs:168,202 `mul w0, w1, w2` / `mul x0, x1, x2`; i128_ops.rs:77 `mul x0, x2, x4`.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- neon_arr_to_q_size accepts 1d/2d, so size==11 encodes (see bugs).
- encode_neon_mul uses dest arrangement only; source T is discarded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..2 / sf / Ra=31 / neon_arr_to_q_size / source T).

---

# Confirmed invariants (encode_msub)

- Valid four-GPR same-width MSUB with Rd/Rn/Rm/Ra in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_msub(Rd, Rn, Rm, ZR) equals llvm-mc `mneg Rd, Rn, Rm` and llvm-mc `msub Rd, Rn, Rm, ZR` (1000 cases). Documented alias at data_processing.rs:677.
- encode_msub(X-ops) XOR encode_msub(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:21]=0011011000, Rm at [20:16], o0=1 at 15, Ra at [14:10], Rn at [9:5], Rd at [4:0].
- `lr` in any of the four slots encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 4 operands, non-register operands (Imm/Symbol/Mem/Shift/Cond/Label), and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `msub x0, x1, x2, x3` encodes as 0x9b028c20; `msub w0, w1, w2, w3` as 0x1b028c20; `msub x0, x1, x2, xzr` / `mneg x0, x1, x2` as 0x9b02fc20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) MSUB: `sf 00 11011 000 Rm 1 Ra Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All four registers same width. Exactly four operands. o0 (bit 15) is 1 (MADD is 0).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:246 msub.
- Callers: alu.rs:178,183,207,211 emit `msub w0, w3, w2, w1` / `msub x0, x3, x2, x1` for remainder.
- Sibling encode_mneg data_processing.rs:677 documents MNEG as MSUB with Ra=XZR.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..3 / sf / parse_reg_num lr / invalid / non-Reg / extra / mixed / SP / FP).

---

# Confirmed invariants (encode_neon_qshrn)

- Valid vector SQSHRN/UQSHRN/SQRSHRN/UQRSHRN (+2) with Ta in {8h,4s,2d}, Tb matching Ta and the 2-suffix, Vd/Vn in v0–v31, shift in [1, dest_esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., is_high=false) XOR encode(..., is_high=true) = 1<<30 (ARM ARM Q bit) (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29 (ARM ARM U bit) (1000 cases).
- encode(..., is_rounding=false) XOR encode(..., is_rounding=true) = 1<<11 (opcode 100101 vs 100111) (1000 cases).
- Success-path word: bit 31=0, Q at 30, U at 29, bits [28:23]=011110, immh:immb at [22:16]=src_esize-shift, opcode at [15:10]=100101/100111, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, unsupported source Ta (not 8h/4s/2d), non-RegArrangement/non-Imm kinds, invalid names (v32, foo, empty, v, v-1), and bare Operand::Reg source always Err.
- Known-answer: `sqshrn v0.8b, v1.8h, #1` encodes as 0x0f0f9420; `#8` as 0x0f089420; `sqshrn2 v0.16b, v1.8h, #1` as 0x4f0f9420; `uqshrn v0.8b, v1.8h, #1` as 0x2f0f9420; `sqrshrn v0.8b, v1.8h, #1` as 0x0f0f9c20; `uqrshrn2 v0.4s, v1.2d, #32` as 0x6f209c20; `sqshrn v0.4h, v1.4s, #16` as 0x0f109420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SQSHRN/UQSHRN/SQRSHRN/UQRSHRN: `0 Q U 011110 immh immb opcode Rn Rd`. opcode 100101 non-rounding / 100111 rounding. U=0 signed / U=1 unsigned. Q=0 lower half / Q=1 (`2` suffix) upper half. dest_esize = 8<<HighestSetBit(immh); shift = 2*esize - UInt(immh:immb) in [1, dest_esize]. Ta/Tb: 8H→8B/16B (1..8), 4S→4H/8H (1..16), 2D→2S/4S (1..32).
- Dispatch: encoder/mod.rs:637-648. Scalar sqshrn (non-arrangement dest) is encode_neon_scalar_qshrn, out of scope.
- Sibling encode_neon_shrn neon.rs:1443-1444 checks `shift > half_bits` with half_bits = source/2.

## Quirks

- Shift range uses source element size (16/32/64), so dest_esize+1 through source_esize encode (see bugs).
- Dest arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg, so GPR/FP dest encodes as Vd (see bugs). Reachable from uqshrn/sqshrn2/sqrshrn/uqrshrn (+2).
- Shift is `get_imm as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Ta / get_imm as u32 / get_neon_reg Reg dest+source).

---

# Confirmed invariants (encode_movz)

- Valid GPR + imm16 + optional lsl (hw in {0,1} for W, {0,1,2,3} for X; Rd=31 is xzr/wzr) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_movz(X-ops) XOR encode_movz(W-ops) at equal rd/imm/hw in {0,1} = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:23]=10100101 (opc=10), hw at [22:21], imm16 at [20:5], Rd at [4:0].
- Constant `:abs_g0:`/`:abs_g1:`/`:abs_g2:`/`:abs_g3:` (and `_nc`) encode the extracted 16-bit chunk and match llvm-mc of the resolved `movz Rd, #chunk [, lsl #shift]` (1000 cases).
- `lr` as Rd encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-imm16 second operands (unknown modifier, non-constant abs_g symbol, Symbol/Label/Mem/Reg) always Err.
- Known-answer: `movz x0, #42` encodes as 0xd2800540; `movz w0, #42` as 0x52800540; `movz x0, #42, lsl #16` as 0xd2a00540.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Move wide (immediate) MOVZ: `sf 10 100101 hw imm16 Rd`. Register 31 is XZR/WZR, never SP/WSP. imm16 in [0, 65535]. hw in {0,1} when sf=0; {0,1,2,3} when sf=1. Semantics: Rd := ZeroExtend(imm16) << (hw*16).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:220 movz.
- Callers: emit.rs:911-922 movz Rd, #imm16 [, lsl #N] as the start of MOVZ+MOVK sequences.
- `:abs_g*:` modifiers are documented for movz/movk (data_processing.rs:179-181).

## Quirks

- Extra operands beyond the optional lsl are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Immediate is masked with `(imm as u32) & 0xFFFF` with no range check (see bugs).
- Non-lsl shift kinds default to hw=0; lsl amount is integer-divided by 16 with no range check (see bugs).
- Unresolved abs_g symbols (non-constant) fall through to get_imm and Err; RelocType has no MOVW variants.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg / Modifier abs_g / get_imm / Shift lsl vs other / extra non-Shift / too few / FP / invalid name).

---

# Confirmed invariants (encode_movn)

- Valid GPR + imm16 + optional lsl (hw in {0,1} for W, {0,1,2,3} for X; Rd=31 is xzr/wzr) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_movn(X-ops) XOR encode_movn(W-ops) at equal rd/imm/hw in {0,1} = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:23]=00100101 (opc=00), hw at [22:21], imm16 at [20:5], Rd at [4:0].
- `lr` as Rd encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-imm16 second operands (unknown modifier, non-constant abs_g symbol, Symbol/Label/Mem/Reg) always Err.
- Known-answer: `movn x0, #42` encodes as 0x92800540; `movn w0, #42` as 0x12800540; `movn x0, #42, lsl #16` as 0x92a00540.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Move wide (immediate) MOVN: `sf 00 100101 hw imm16 Rd`. Register 31 is XZR/WZR, never SP/WSP. imm16 in [0, 65535]. hw in {0,1} when sf=0; {0,1,2,3} when sf=1. Semantics: Rd := NOT(ZeroExtend(imm16) << (hw*16)).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:222 movn.
- Callers: emit.rs:873-902 movn Rd, #imm16 [, lsl #N] as the start of MOVN+MOVK sequences.
- `:abs_g*:` modifiers are documented for movz/movk only (data_processing.rs:179-181); encode_movn has no Modifier path.

## Quirks

- Extra operands beyond the optional lsl are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Immediate is masked with `(imm as u32) & 0xFFFF` with no range check (see bugs).
- Non-lsl shift kinds default to hw=0; lsl amount is integer-divided by 16 with no range check (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg / get_imm / Shift lsl vs other / extra non-Shift / too few / FP / invalid name).

---

# Confirmed invariants (encode_movk)

- Valid GPR + imm16 + optional lsl (hw in {0,1} for W, {0,1,2,3} for X; Rd=31 is xzr/wzr) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_movk(X-ops) XOR encode_movk(W-ops) at equal rd/imm/hw in {0,1} = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:23]=11100101, hw at [22:21], imm16 at [20:5], Rd at [4:0].
- Constant `:abs_g0:`/`:abs_g1:`/`:abs_g2:`/`:abs_g3:` (and `_nc`) encode the extracted 16-bit chunk and match llvm-mc of the resolved `movk Rd, #chunk [, lsl #shift]` (1000 cases).
- `lr` as Rd encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-imm16 second operands (unknown modifier, non-constant abs_g symbol, Symbol/Label/Mem/Reg) always Err.
- Known-answer: `movk x0, #42` encodes as 0xf2800540; `movk w0, #42` as 0x72800540; `movk x0, #42, lsl #16` as 0xf2a00540.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Move wide (immediate) MOVK: `sf 11 100101 hw imm16 Rd`. Register 31 is XZR/WZR, never SP/WSP. imm16 in [0, 65535]. hw in {0,1} when sf=0; {0,1,2,3} when sf=1.
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:221 movk.
- Callers: emit.rs:906-928 movk Rd, #imm16 [, lsl #N]; intrinsics.rs:182-184.

## Quirks

- Extra operands beyond the optional lsl are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Immediate is masked with `(imm as u32) & 0xFFFF` with no range check (see bugs).
- Non-lsl shift kinds default to hw=0; lsl amount is integer-divided by 16 with no range check (see bugs).
- Unresolved abs_g symbols (non-constant) fall through to get_imm and Err; RelocType has no MOVW variants.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg / Modifier abs_g / get_imm / Shift lsl vs other / extra non-Shift / too few).

---

# Confirmed invariants (encode_madd)

- Valid four-GPR same-width MADD with Rd/Rn/Rm/Ra in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_madd(Rd, Rn, Rm, ZR) equals llvm-mc `mul Rd, Rn, Rm` and llvm-mc `madd Rd, Rn, Rm, ZR` (1000 cases). Documented alias at data_processing.rs:589.
- encode_madd(X-ops) XOR encode_madd(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:21]=0011011000, Rm at [20:16], o0=0 at 15, Ra at [14:10], Rn at [9:5], Rd at [4:0].
- `lr` in any of the four slots encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 4 operands, non-register operands (Imm/Symbol/Mem/Shift/Cond/Label), and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `madd x0, x1, x2, x3` encodes as 0x9b020c20; `madd w0, w1, w2, w3` as 0x1b020c20; `madd x0, x1, x2, xzr` / `mul x0, x1, x2` as 0x9b027c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) MADD: `sf 00 11011 000 Rm 0 Ra Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All four registers same width. Exactly four operands. o0 (bit 15) is 0 (MSUB is 1).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:245 madd.
- Callers: i128_ops.rs:79-80 emit `madd x1, x3, x4, x1` / `madd x1, x2, x5, x1`.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..3 / sf / parse_reg_num lr / invalid / non-Reg).

---

# Confirmed invariants (encode_logical)

- Valid AND/ORR/EOR/ANDS shifted-register with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr, shift in {lsl,lsr,asr,ror} with amount in [0,31] (W) or [0,63] (X), matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid AND/ORR/EOR/ANDS bitmask-immediate constructed from ARM ARM (size, ones, immr), Rd=31 as SP (AND/ORR/EOR) or XZR (ANDS), Rn=31 as XZR, matches llvm-mc (1000 cases).
- Valid NEON AND/ORR/EOR with T in {8b,16b}, Vd/Vn/Vm in v0–v31, matches llvm-mc (1000 cases).
- encode(opc_a) XOR encode(opc_b) = (opc_a XOR opc_b)<<29 at equal other fields (ARM ARM opc at bits [30:29]) (1000 cases).
- encode(X) XOR encode(W) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path shifted-register word: sf at 31, opc at [30:29], bits [28:24]=01010, shift at [23:22], N=0 at 21, Rm at [20:16], imm6 at [15:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, invalid bitmask (0 / all-ones / 0x1234 / 0x5 / 0x1001), third operand that is Symbol/Mem/Label/Cond, and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `and x0, x1, x2` encodes as 0x8a020020; `orr x0, x1, x2` as 0xaa020020; `eor x0, x1, x2` as 0xca020020; `ands x0, x1, x2` as 0xea020020; `and w0, w1, w2` as 0x0a020020; `and x0, x1, #1` as 0x92400020; `and sp, x0, #1` as 0x9240001f; `and v0.16b, v1.16b, v2.16b` as 0x4e221c20; `and v0.8b, v1.8b, v2.8b` as 0x0e221c20; `orr v0.16b, v1.16b, v2.16b` as 0x4ea21c20; `eor v0.16b, v1.16b, v2.16b` as 0x6e221c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Logical (shifted register): `sf opc 01010 shift N Rm imm6 Rn Rd` with N=0. opc 00 AND / 01 ORR / 10 EOR / 11 ANDS.
- ARM ARM Logical (immediate): `sf opc 100100 N immr imms Rn Rd`. Rd=31 is SP for AND/ORR/EOR and XZR for ANDS (TST). llvm-mc rejects SP as Rn.
- ARM ARM Advanced SIMD logical: `0 Q U 01110 size 1 Rm 000111 Rn Rd`. AND U=0 size=00; ORR U=0 size=10; EOR U=1 size=00. T in {8B,16B} only.
- Dispatch: encoder/mod.rs:231-234 and/orr/eor/ands.
- `lr` is a 64-bit alias of X30.

## Quirks

- Extra operands beyond a shift at index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so shifted-register SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Shift amount is masked with 0x3F with no range check (see bugs).
- Unknown shift kinds default to LSL (see bugs).
- NEON T other than 16b is encoded with Q=0; source arrangements discarded (see bugs).
- ANDS (opc=11) on NEON encodes as EOR-like (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / NEON / Imm / Reg / unsupported-third / invalid-reg / sf).

---

# Confirmed invariants (encode_ldxr_stxr)

- Valid LDXR/STXR/LDXRB/STXRB/LDXRH/STXRH with Rt in x0–x30/xzr or w0–w30/wzr (byte/half always W), Rn in x0–x30/sp, Ws in w0–w30/wzr not aliasing Rt/Xn, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(load) XOR encode(store with Ws=31) at equal Rt/Rn = 1<<22 (ARM ARM L bit) (1000 cases).
- encode(X) XOR encode(W) at equal register numbers = 1<<30 (ARM ARM size 11 vs 10) (1000 cases).
- encode(byte) XOR encode(half) at equal W registers = 1<<30 (ARM ARM size 00 vs 01) (1000 cases).
- Success-path word: bits [29:24]=001000, bit 23=0, bit 21=0 (not pair), o0=0 at bit 15, Rt2=11111 at [14:10], size at [31:30], L at 22, Rs=31 on load else Ws at [20:16], Rn at [9:5], Rt at [4:0].
- Fewer than 2 (load) / 3 (store) operands, non-Reg first operand, non-Mem memory slot (Imm/Symbol/pre/post-index), and invalid base names (foo, x32) always Err.
- Known-answer: `ldxr x0, [x1]` encodes as 0xc85f7c20; `ldxr w0, [x1]` as 0x885f7c20; `ldxrb w0, [x1]` as 0x085f7c20; `ldxrh w0, [x1]` as 0x485f7c20; `stxr w0, x1, [x2]` as 0xc8007c41; `stxr w0, w1, [x2]` as 0x88007c41; `ldxr x0, [sp]` as 0xc85f7fe0; `ldxr lr, [x2]` as 0xc85f7c5e.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Load/Store Exclusive (single): `size 001000 0 L 0 Rs o0 Rt2 Rn Rt`. size=00 byte / 01 half / 10 word / 11 doubleword. Offset absent or #0. o0=0 distinguishes LDXR/STXR from LDAXR/STLXR.
- Rt register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldxr sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- STXR Ws is Wt (31=WZR); llvm-mc rejects Xt/SP as status and rejects Ws aliasing Rt/Xn ("status is also a source"). WZR vs SP is allowed.
- Byte/half data is Wt (llvm-mc rejects `ldxrb x0, [x1]`).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:348-353 ldxr/stxr/ldxrb/stxrb/ldxrh/stxrh.

## Quirks

- Extra operands beyond the memory slot are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `ldxr sp, ...` encodes as ZR (see bugs).
- W-register base is accepted and encoded as the same-number X register (see bugs).
- XZR as base encodes as SP (register 31) (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- X as STXR status is accepted; only the number is used (see bugs).
- forced_size byte/half with an X data register is encoded (see bugs).
- `Mem { base, .. }` ignores a nonzero offset (see bugs).
- STXR Ws overlapping Rt/Rn is encoded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (is_load / get_reg miss / non-Mem / parse_reg_num None / forced_size / is_64).

---

# Confirmed invariants (encode_neon_float_three_same)

- Valid vector FP three-same (FADD/FSUB/FMUL/FDIV/FMAX/FMIN/FMAXNM/FMINNM/FMLA/FMLS/FRECPS/FRSQRTS/FCMEQ/FCMGE/FCMGT/FACGE/FACGT/FABD) with T in {2s,4s,2d}, Vd/Vn/Vm in v0–v31, and ARM-correct (U, size_hi, opcode) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., U=0) XOR encode(..., U=1) = 1<<29 (ARM ARM U bit) (1000 cases).
- encode(..., size_hi=0) XOR encode(..., size_hi=1) = 1<<23 (ARM ARM size[1]) (1000 cases).
- encode(2s) XOR encode(4s) at equal register numbers = 1<<30 (ARM ARM Q bit) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T (2s→0, 4s/2d→1), U at 29, bits [28:24]=01110, size at [23:22]=(size_hi<<1)|sz, bit 21=1, Rm at [20:16], opcode at [15:11], bit 10=1, Rn at [9:5], Rd at [4:0].
- Arrangement other than 2s/4s/2d, fewer than 3 operands, non-register dest/src/Vm, invalid names (v32, foo, empty, v, v-1, v99), and dest Operand::Reg (no arrangement) always Err.
- Known-answer: `fadd v0.4s, v1.4s, v2.4s` encodes as 0x4e22d420; `fadd v0.2s, v1.2s, v2.2s` as 0x0e22d420; `fadd v0.2d, v1.2d, v2.2d` as 0x4e62d420; `fsub v0.4s, v1.4s, v2.4s` as 0x4ea2d420; `fmul v0.4s, v1.4s, v2.4s` as 0x6e22dc20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD three-same FP: `0 Q U 01110 size 1 Rm opcode 1 Rn Rd`. size[1]=size_hi, size[0]=sz (0=single, 1=double).
- Valid T is 2S, 4S, 2D (llvm-mc rejects 8b/16b/4h/8h/1d without +fullfp16).
- Scalar `fadd s0, s1, s2` / `fadd d0, d1, d2` is a different encoding (scalar FP) — not this vector helper.
- Exactly three operands (llvm-mc rejects a fourth).
- Dispatch: encoder/mod.rs:377-496 fadd/fsub/fmul/fdiv/fmax/fmin/fmaxnm/fminnm/fmla/fmls/frecps/frsqrts/fcmeq/fcmge/fcmgt/facge/facgt.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded; dest T is used (see bugs). Operand::Reg source (empty arrangement) is accepted (see bugs).
- parse_reg_num accepts x/w/d/s/q/v/h/b prefixes, so non-V names encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_neon_reg dest/Vn/Vm, match 2s/4s/2d/_, arity).

---

# Confirmed invariants (encode_ldxp_stxp)

- Valid LDXP/LDAXP/STXP/STLXP with Rt/Rt2 in x0–x30/xzr or w0–w30/wzr, Rn in x0–x30/sp, Ws in w0–w30/wzr not aliasing Rt/Rt2/Xn, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_ldxp_stxp(..., acqrel=true) XOR encode_ldxp_stxp(..., acqrel=false) = 1<<15 (ARM ARM o0 bit) (1000 cases).
- encode_ldxp_stxp(X-ops) XOR encode_ldxp_stxp(W-ops) = 1<<30 at equal register numbers (ARM ARM sz) (1000 cases).
- Success-path word: bit 31=1, size 11/10 at [31:30], bits [29:24]=001000, bit 23=0, L at 22, o1=1 at 21, Rs=31 on load else Ws at [20:16], o0 at 15, Rt2 at [14:10], Rn at [9:5], Rt at [4:0].
- Fewer than 3 (load) / 4 (store) operands, non-Reg first operand, non-Mem memory slot (Imm/Symbol/pre/post-index), and invalid base names (foo, x32) always Err.
- Known-answer: `ldxp x0, x1, [x2]` encodes as 0xc87f0440; `ldxp w0, w1, [x2]` as 0x887f0440; `ldaxp x0, x1, [x2]` as 0xc87f8440; `stxp w0, x1, x2, [x3]` as 0xc8200861; `stlxp w0, x1, x2, [x3]` as 0xc8208861; `ldxp x0, x1, [sp]` as 0xc87f07e0; `ldxp lr, x1, [x2]` as 0xc87f045e.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Load/Store Exclusive Pair: `size 001000 0 L 1 Rs o0 Rt2 Rn Rt`. size=10 (W pair) / 11 (X pair). Offset absent or #0.
- Rt/Rt2 register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldxp sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- STXP Ws is Wt (31=WZR); llvm-mc rejects Xt/SP as status and rejects Ws aliasing Rt/Rt2/Xn ("status is also a source"). WZR vs SP is allowed.
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:366-369 ldxp/ldaxp/stxp/stlxp.

## Quirks

- Extra operands beyond the memory slot are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `stxp w0, sp, ...` encodes as ZR (see bugs).
- W-register base is accepted and encoded as the same-number X register (see bugs).
- XZR as base encodes as SP (register 31) (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- sz is taken only from the first data register; mixed X/W encodes (see bugs).
- X as STXP status is accepted; only the number is used (see bugs).
- `Mem { base, .. }` ignores a nonzero offset (see bugs).
- STXP Ws overlapping Rt/Rt2/Rn is encoded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (too-few / non-Reg / non-Mem / parse_reg_num None).

---

# Confirmed invariants (encode_ldur_stur)

- Valid GPR LDUR/STUR/LDTR/STTR with Rt in x0–x30/xzr or w0–w30/wzr, Rn in x0–x30/sp, offset in [-256, 255] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid SIMD LDUR/STUR with Rt in b/h/s/d/q 0–31, Rn in x0–x30/sp, offset in [-256, 255] matches llvm-mc (1000 cases).
- encode_ldur_stur(..., is_load=true) XOR encode_ldur_stur(..., is_load=false) = 1<<22 (ARM ARM opc bit) (1000 cases).
- encode_ldur_stur(..., op2=00) XOR encode_ldur_stur(..., op2=10) = 1<<11 over GPR (ARM ARM unscaled vs unprivileged) (1000 cases).
- Success-path word: bits [29:27]=111, bits [25:24]=00, bit 21=0, imm9 at [20:12], op2 at [11:10], Rn at [9:5], Rt at [4:0]; GPR size 11/10 from X/W; SIMD size/opc from B/H/S/D/Q.
- Fewer than 2 operands, non-Reg first operand, non-Mem second operand (Imm/Symbol/pre/post-index), and invalid base names (foo, x32) always Err.
- Known-answer: `ldur x0, [x1]` encodes as 0xf8400020; `ldur q0, [x1]` as 0x3cc00020; `ldtr x0, [x1]` as 0xf8400820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM unscaled LDUR/STUR: `size 111 V 00 opc 0 imm9 00 Rn Rt`. LDTR/STTR: bits [11:10]=10. simm9 in [-256, 255].
- Rt register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldur sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- SIMD Rt is valid for LDUR/STUR only (llvm-mc rejects `ldtr d0, ...` and `ldur v0, ...`).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg / encode_ldr_str_auto).
- Dispatch: encoder/mod.rs:336-339 ldur/stur/ldtr/sttr.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- imm9 is masked with 0x1FF with no range check (see bugs).
- parse_reg_num maps sp/wsp to 31, so `stur sp, [x0]` encodes as `stur wzr, [x0]` (see bugs).
- W-register base is accepted and encoded as the same-number X register (see bugs).
- XZR as base encodes as SP (register 31) (see bugs).
- SIMD Rt on LDTR/STTR is encoded (see bugs).
- V-register Rt falls through to size=11 opc=01 (D form) (see bugs).
- `lr` is sized as 32-bit because size uses `starts_with('x')` (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / non-Mem / parse_reg_num None / lr alias).

---

# Confirmed invariants (encode_neon_sli)

- Valid vector SLI with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [0, esize(T)-1] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_neon_sli(Tlo) XOR encode_neon_sli(Thi) = 1<<30 for (8b,16b)/(4h,8h)/(2s,4s) at equal shift (ARM ARM Q bit) (1000 cases).
- encode_neon_sli(shift+1) − encode_neon_sli(shift) = 1<<16 when both shifts are in range (ARM ARM immh:immb = esize + shift) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T, U=1 at 29, bits [28:23]=011110, immh:immb at [22:16]=esize+shift, bits [15:10]=010101, Rn at [9:5], Rd at [4:0].
- Arrangement other than 8b/16b/4h/8h/2s/4s/2d (including 1d), fewer than 3 operands, non-register dest/src (Imm/Mem/Symbol/Shift/Cond/Label), invalid names (v32, foo, empty, v, v-1, v99), dest Operand::Reg (no arrangement), and non-Imm shift always Err.
- Known-answer: `sli v0.8b, v1.8b, #0` encodes as 0x2f085420; `sli v0.8b, v1.8b, #7` as 0x2f0f5420; `sli v0.16b, v1.16b, #3` as 0x6f0b5420; `sli v0.4h, v1.4h, #15` as 0x2f1f5420; `sli v0.2d, v1.2d, #0` as 0x6f405420; `sli v0.2d, v1.2d, #63` as 0x6f7f5420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SLI: `0 Q 1 011110 immh:immb 010101 Rn Rd`. Valid T: 8B/16B (shift 0..7), 4H/8H (0..15), 2S/4S (0..31), 2D (0..63). 1D reserved. Scalar `sli d0, d1, #0` is a different encoding (bits[31:30]=01) — not this vector helper.
- Exactly three operands (llvm-mc rejects a fourth).
- Dispatch: encoder/mod.rs:661 `"sli" => encode_neon_sli(operands)`.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded; dest T is used (see bugs). Operand::Reg source (empty arrangement) is accepted (see bugs).
- parse_reg_num accepts x/w/d/s/q/v/h/b prefixes, so non-V names encode as V registers (see bugs).
- Shift is `get_imm as u32` then `(esize + shift) & mask`: negative panics in debug / wraps in release; shift >= esize encodes reserved immh=0000 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_neon_reg Operand::Reg dest / src).

---

# Confirmed invariants (encode_neon_float_cmp_zero)

- Valid vector FCMEQ/FCMGE/FCMGT/FCMLE/FCMLT-to-zero with T in {2s,4s,2d}, Vd/Vn in v0–v31, and ARM-correct (U, size_hi=1, opcode) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_neon_float_cmp_zero(ops, 0, size_hi, opcode) XOR encode_neon_float_cmp_zero(ops, 1, size_hi, opcode) = 1<<29 (ARM ARM U bit) (1000 cases).
- encode_neon_float_cmp_zero(ops, U, 0, opcode) XOR encode_neon_float_cmp_zero(ops, U, 1, opcode) = 1<<23 (size_hi = size[1]) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T (2s→0, 4s/2d→1), U at 29, bits [28:24]=01110, size at [23:22]=(size_hi<<1)|sz, bits [21:17]=10000, opcode at [16:12], bits [11:10]=10, Rn at [9:5], Rd at [4:0].
- Arrangement other than 2s/4s/2d, fewer than 2 operands, non-register dest/src, invalid names (v32, foo, empty, v, v-1, v99), and dest Operand::Reg (no arrangement) always Err.
- Known-answer: `fcmeq v0.4s, v1.4s, #0.0` encodes as 0x4ea0d820; `fcmge v0.4s, v1.4s, #0.0` as 0x6ea0c820; `fcmlt v0.4s, v1.4s, #0.0` as 0x4ea0e820; `fcmeq v0.2s, v1.2s, #0.0` as 0x0ea0d820; `fcmeq v0.2d, v1.2d, #0.0` as 0x4ee0d820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM-correct (U, size_hi, opcode): FCMEQ (0,1,01101), FCMGE (1,1,01100), FCMGT (0,1,01100), FCMLE (1,1,01101), FCMLT (0,1,01110). size = 1sz (not 0sz).
- Valid T is 2S, 4S, 2D (llvm-mc rejects 8b/16b/4h/8h/1d without +fullfp16; 4h/8h is a different FP16 encoding).
- Scalar `fcmeq s0, s1, #0.0` is a different encoding (bits[31:30]=01) — not this vector helper.
- Dispatch passes [Vd, Vn, Imm(0)] for fcmeq/fcmge/fcmgt #0.0; the helper reads only [0] and [1].

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded; dest T is used (see bugs).
- parse_reg_num accepts x/w/d/s/q/v/h/b prefixes, so non-V names encode as V registers (see bugs).
- Function comment says size=0sz; ARM ARM and llvm-mc use size=1sz. The helper packs the caller-supplied size_hi; dispatcher currently passes size_hi=0 for fcmeq/fcmge/fcmle (out of this symbol's scope).
- Dispatcher passes opcode=01101 for fcmlt; ARM/llvm-mc use 01110 (out of this symbol's scope).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_neon_reg Operand::Reg dest / non-V prefix).

---

# Confirmed invariants (encode_neon_across_long)

- Valid SADDLV/UADDLV with dest V matching T (H for 8B/16B, S for 4H/8H, D for 4S) and Vn in v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_neon_across_long(ops, 0, 0b00011) XOR encode_neon_across_long(ops, 1, 0b00011) = 1<<29 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T, U at 29, bits [28:24]=01110, size at [23:22] from T, bits [21:17]=11000, bits [16:12]=00011, bits [11:10]=10, Rn at [9:5], Rd at [4:0].
- Fewer than 2 operands, non-register dest/src (Imm/Mem/Symbol/Shift/Cond/Label), and invalid names (v32, h32, foo, empty, v, v-1) always Err — including dest RegArrangement with an invalid name.
- Known-answer: `saddlv h0, v1.8b` encodes as 0x0e303820; `uaddlv h0, v0.8b` as 0x2e303800; `saddlv d0, v1.4s` as 0x4eb03820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- SADDLV/UADDLV dest V is H/S/D matching T (llvm-mc rejects b/q/x/w/v dest and vector-arrangement dest).
- Valid T is 8B, 16B, 4H, 8H, 4S (llvm-mc rejects 2S/1D/2D and unknown qualifiers).
- Exactly two operands (llvm-mc rejects a third operand).
- Codegen emits `uaddlv h0, v0.8b` (alu.rs:65).

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- neon_arr_to_q_size accepts 2s/1d/2d, so reserved T is encoded (see bugs).
- Dest prefix is ignored; only parse_reg_num is used, so b/s/d/q/x/w/v dest encode as the matching-number H/S/D form (see bugs).
- Operand::RegArrangement dest is accepted and its arrangement discarded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (dest `_` non-Reg / parse_reg_num None on dest RegArrangement).

---

# Confirmed invariants (encode_ldar_stlr)

- Valid LDAR/STLR/LDARB/STLRB/LDARH/STLRH with Wt/Xt Rt (31=XZR/WZR) and Xn|SP base, offset 0, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_ldar_stlr(ops, true, sz) XOR encode_ldar_stlr(ops, false, sz) = 1<<22 (ARM ARM L bit) (1000 cases).
- Success-path word: size at [31:30], bits [29:24]=001000, bit 23=1, L at 22, bit 21=0, Rs=31 at [20:16], o0=1 at 15, Rt2=31 at [14:10], Rn at [9:5], Rt at [4:0].
- Fewer than 2 operands, non-Reg first operand, non-Mem second operand (Imm/Symbol/pre/post/reg-offset), and invalid base names (foo, x32) always Err.
- Known-answer: `ldar x0, [x1]` encodes as 0xc8dffc20; `stlr x0, [x1]` as 0xc89ffc20; `ldar w0, [x1]` as 0x88dffc20; `ldarb w0, [x1]` as 0x08dffc20; `ldarh w0, [x1]` as 0x48dffc20; `ldar xzr, [sp]` as 0xc8dfffff.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- LDAR/STLR Rt register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldar sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- Offset must be absent or #0 (llvm-mc: "index must be absent or #0").
- Byte/halfword forms take Wt only (llvm-mc rejects `ldarb x0, [x1]`).
- LDAR/STLR take GPR only (llvm-mc rejects `ldar d0, ...`).
- Mixed W data + X base is valid (`ldar w0, [x1]`).

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `stlr sp, [x0]` encodes as `stlr xzr, [x0]` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- W-register base is accepted and encoded as the same number X register (see bugs).
- XZR as base encodes as SP (register 31) (same property as W-base).
- `Mem { base, .. }` ignores a nonzero offset (same property as W-base).
- Xt for ldarb/ldarh/stlrb/stlrh is accepted (size forced; Rt number still encoded).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg non-Reg / parse_reg_num None).

---

# Confirmed invariants (encode_eon)

- Same-width GPR EON (x0–x30/xzr/lr and w0–w30/wzr, optional lsl/lsr/asr/ror with amount in [0,31] W / [0,63] X) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_eon(ops) XOR encode_logical(ops, 0b10) = 1<<21 (ARM ARM EON N=1 vs EOR N=0) (1000 cases).
- Success-path word: sf at 31 from Rd width, opc=10 at [30:29], bits [28:24]=01010, shift at [23:22], N=1 at 21, Rm at [20:16], imm6 at [15:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x99) always Err.
- Known-answer: `eon x0, x1, x2` encodes as 0xca220020; `eon w0, w1, w2` as 0x4a220020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- EON register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `eon sp, ...`).
- EON takes Wt/Xt only (llvm-mc rejects `eon d0, ...`).
- llvm-mc rejects mixed x/w, a 4th non-shift operand, shift amount 32 (W) / 64 (X), and unknown shift kinds.
- llvm-mc accepts `eon Rd, Rn, #imm` as the assembler alias of `eor Rd, Rn, #~imm` (Rd may not be ZR).
- llvm-mc omits `lsl #0` in disassembly of unshifted EON.

## Quirks

- Extra operands beyond a non-Shift index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `eon wsp, ...` encodes as `eon wzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- Shift amount is masked with 0x3F with no width check (see bugs).
- Unknown shift kinds default to LSL via `_ => 0b00` (see bugs).
- No immediate path: get_reg on operand 2 rejects `eon Rd, Rn, #imm` (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (unknown shift `_ => 0b00` / parse_reg_num None).

---

# Confirmed invariants (encode_div)

- Same-width GPR UDIV/SDIV (x0–x30/xzr and w0–w30/wzr, including register 31 as XZR/WZR) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_div(ops, true) XOR encode_div(ops, false) = 1<<10 (ARM ARM UDIV o1=0 vs SDIV o1=1) (1000 cases).
- Success-path word: sf at 31 from Rd width, bit 30=0, S=0 at 29, bits [28:21]=0b11010110, Rm at [20:16], bits [15:11]=00001, o1 at 10, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Non-register operands (Imm/Mem/Symbol/Shift/Cond) in any of the three slots always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Known-answer: `udiv x0, x1, x2` encodes as 0x9ac20820; `sdiv w0, w1, w2` as 0x1ac20c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- UDIV/SDIV register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `udiv sp, ...`).
- UDIV/SDIV take Wt/Xt only (llvm-mc rejects `udiv d0, ...`).
- llvm-mc rejects mixed x/w and a fourth operand.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `sdiv wsp, ...` encodes as `sdiv wzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg None / parse_reg_num None / FP prefixes).

---

# Confirmed invariants (encode_csneg)

- Same-width GPR CSNEG (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csneg(ops) XOR encode_csinc(ops) = 1<<30 (ARM ARM CSNEG op=1 vs CSINC op=0) (1000 cases).
- encode_csneg([Rd, Rn, Rn, invert(cond)]) equals encode_cneg([Rd, Rn, cond]) over Cond14 (ARM ARM CNEG alias of CSNEG) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csneg x0, x1, x2, eq` encodes as 0xda820420; `csneg w0, w1, w2, ne` as 0x5a821420; `csneg x0, x1, x2, al` as 0xda82e420; `csneg x0, x1, x1, ne` as 0xda811420 (CNEG alias).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSNEG register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csneg sp, ...`).
- CSNEG takes Wt/Xt only (llvm-mc rejects `csneg d0, ...`).
- Cond AL and NV are valid for architectural CSNEG (unlike CNEG alias).
- llvm-mc rejects mixed x/w and a fifth operand.
- llvm-mc disassembles `csneg x0, x1, x1, ne` as `cneg x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csneg sp, ...` encodes as `csneg xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csinv)

- Same-width GPR CSINV (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csinv(ops) XOR encode_csel(ops) = 1<<30 (ARM ARM CSINV op=1 vs CSEL op=0) (1000 cases).
- encode_csinv([Rd, Rn, Rn, invert(cond)]) equals encode_cinv([Rd, Rn, cond]) over Cond14 (ARM ARM CINV alias of CSINV) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=00 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csinv x0, x1, x2, eq` encodes as 0xda820020; `csinv w0, w1, w2, ne` as 0x5a821020; `csinv x0, x1, x2, al` as 0xda82e020; `csinv x0, x1, x1, ne` as 0xda811020 (CINV alias).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSINV register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csinv sp, ...`).
- CSINV takes Wt/Xt only (llvm-mc rejects `csinv d0, ...`).
- Cond AL and NV are valid for architectural CSINV (unlike CINV/CSETM aliases).
- llvm-mc rejects mixed x/w and a fifth operand.
- llvm-mc disassembles `csinv x0, x1, x1, ne` as `cinv x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csinv sp, ...` encodes as `csinv xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csinc)

- Same-width GPR CSINC (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csinc(ops) XOR encode_csel(ops) = 1<<10 (ARM ARM CSINC op2=01 vs CSEL op2=00) (1000 cases).
- encode_csinc([Rd, Rn, Rn, invert(cond)]) equals encode_cinc([Rd, Rn, cond]) over Cond14 (ARM ARM CINC alias of CSINC) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csinc x0, x1, x2, eq` encodes as 0x9a820420; `csinc w0, w1, w2, ne` as 0x1a821420; `csinc x0, x1, x2, al` as 0x9a82e420; `csinc x0, x1, x1, ne` as 0x9a811420 (CINC alias).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSINC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csinc sp, ...`).
- CSINC takes Wt/Xt only (llvm-mc rejects `csinc d0, ...`).
- Cond AL and NV are valid for architectural CSINC (unlike CINC/CSET aliases).
- llvm-mc rejects mixed x/w and a fifth operand.
- llvm-mc disassembles `csinc x0, x1, x1, ne` as `cinc x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csinc sp, ...` encodes as `csinc xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csetm)

- Same-width GPR CSETM (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases, excluding al/nv) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csetm([Rd, cond]) equals encode_csinv([Rd, ZR, ZR, invert(cond)]) (ARM ARM CSETM alias of CSINV) (1000 cases).
- encode_csetm([Rd, cond]) equals encode_cinv([Rd, ZR, cond]) (CINV with Rn=ZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=31 at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=00 at [11:10], Rn=31 at [9:5], Rd at [4:0].
- Fewer than 2 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in either slot always Err.
- Known-answer: `csetm x0, eq` encodes as 0xda9f13e0; `csetm w0, ne` as 0x5a9f03e0; `csinv x0, xzr, xzr, ne` disassembles as the same CSETM.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSETM register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csetm sp, ...`).
- CSETM takes Wt/Xt only (llvm-mc rejects `csetm d0, ...`).
- Cond AL and NV are invalid for the CSETM alias (unlike architectural CSEL).
- llvm-mc rejects a third operand.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- encode_cond accepts al/nv and invert(cond)=cond XOR 1 is applied with no AL/NV guard (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csetm sp, eq` encodes as `csetm xzr, eq` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cset)

- Same-width GPR CSET (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases, excluding al/nv) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cset([Rd, cond]) equals encode_csinc([Rd, ZR, ZR, invert(cond)]) (ARM ARM CSET alias of CSINC) (1000 cases).
- encode_cset([Rd, cond]) equals encode_cinc([Rd, ZR, cond]) (CINC with Rn=ZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=31 at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=01 at [11:10], Rn=31 at [9:5], Rd at [4:0].
- Fewer than 2 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in either slot always Err.
- Known-answer: `cset x0, eq` encodes as 0x9a9f17e0; `cset w0, ne` as 0x1a9f07e0; `csinc x0, xzr, xzr, ne` disassembles as the same CSET.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSET register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cset sp, ...`).
- CSET takes Wt/Xt only (llvm-mc rejects `cset d0, ...`).
- Cond AL and NV are invalid for the CSET alias (unlike architectural CSEL).
- llvm-mc rejects a third operand.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- encode_cond accepts al/nv and invert(cond)=cond XOR 1 is applied with no AL/NV guard (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cset sp, eq` encodes as `cset xzr, eq` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csel)

- Same-width GPR CSEL (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csel(ops) XOR encode_csinc(ops) = 1<<10 (ARM ARM CSEL op2=00 vs CSINC op2=01) (1000 cases).
- encode_csel(ops) XOR encode_csinv(ops) = 1<<30 (ARM ARM CSEL op=0 vs CSINV op=1) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=00 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csel x0, x1, x2, eq` encodes as 0x9a820020; `csel w0, w1, w2, ne` as 0x1a821020; `csel x0, x1, x2, al` as 0x9a82e020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSEL register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csel sp, ...`).
- CSEL takes Wt/Xt only (llvm-mc rejects `csel d0, ...`).
- Cond AL and NV are valid for architectural CSEL (unlike CINC/CINV/CNEG aliases).
- llvm-mc rejects mixed x/w and a fifth operand.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csel sp, ...` encodes as `csel xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cneg)

- Same-width GPR CNEG (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cneg([Rd, Rn, cond]) equals encode_csneg([Rd, Rn, Rn, invert(cond)]) (ARM ARM CNEG alias of CSNEG) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=Rn at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the three slots always Err.
- Known-answer: `cneg x0, x1, eq` encodes as 0xda811420; `cneg w0, w1, ne` as 0x5a810420; `cneg x0, x1, eq` equals `csneg x0, x1, x1, ne`.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CNEG register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cneg sp, ...`).
- CNEG takes Wt/Xt only (llvm-mc rejects `cneg d0, ...`).
- Cond AL and NV are invalid for the CNEG alias (llvm-mc: "condition codes AL and NV are invalid for this instruction").
- llvm-mc rejects mixed x/w and a fourth operand.
- llvm-mc disassembles `csneg x0, x1, x1, ne` as `cneg x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- encode_cond accepts al/nv, so CNEG AL/NV encodes as CSNEG with inverted cond (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cneg sp, ...` encodes as `cneg xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None, get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cmp)

- Same-width GPR CMP immediate form (x0–x30/sp/lr and w0–w30/wsp, imm in unshifted 0..4095 or N<<12 with N in 1..4095, plus explicit lsl #12) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-width GPR CMP shifted-register form (x0–x30/xzr/lr and w0–w30/wzr, lsl/lsr/asr in range) matches llvm-mc (1000 cases).
- Extended-register CMP (sxtw/uxtw/sxtx/uxtx, amount 0..4 including bounds) matches llvm-mc (1000 cases).
- Negative immediate `cmp Rn, #-N` for N in 1..4095 matches llvm-mc's gas rewrite to `cmn Rn, #N` (1000 cases).
- encode_cmp(ops) equals encode_add_sub([ZR] ++ ops, is_sub=true, set_flags=true) where ZR is WZR if Rn is 32-bit else XZR (1000 cases).
- Success-path immediate word: Rd=31, S=1, op=1, bits[28:24]=0b10001, sf from Rn width, unshifted imm12, Rn at [9:5].
- 0 or 1 operands always Err.
- Non-register first operand (Imm/Symbol/Mem/Cond/Shift) with a second operand always Err.
- Known-answer: `cmp x0, #42` encodes as 0xf100a81f; `cmp w0, #42` as 0x7100a81f; `cmp x0, x1` as 0xeb01001f; `cmp sp, #0` as 0xf10003ff; `cmp x0, w1, sxtw` as 0xeb21c01f; `cmp x0, #-1` as 0xb100041f (`cmn x0, #1`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CMP immediate-form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `cmp xzr, #0`).
- CMP shifted-register Rm=31 is XZR/WZR; SP as Rm without extend is rejected by llvm-mc.
- Known-answer: `subs xzr, x0, #42` disassembles as `cmp x0, #42` with the same encoding.

## Quirks

- Extra operands beyond a Shift/Extend are ignored (see bugs).
- encode_cmp does not reject XZR/WZR as immediate-form Rn, so `cmp xzr, #0` encodes as `cmp sp, #0` (see bugs).
- sf is taken from the prepended ZR; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp to 31, so `cmp x0, sp` encodes as `cmp x0, xzr` (see bugs).
- encode_add_sub negates a negative Imm with `-imm_signed`, which panics on i64::MIN in debug (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (non-Reg first operand, extended-register form, negative-imm rewrite).

---

# Confirmed invariants (encode_cmn)

- Same-width GPR CMN immediate form (x0–x30/sp/lr and w0–w30/wsp, imm in unshifted 0..4095 or N<<12 with N in 1..4095, plus explicit lsl #12) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-width GPR CMN shifted-register form (x0–x30/xzr/lr and w0–w30/wzr, lsl/lsr/asr in range) matches llvm-mc (1000 cases).
- Extended-register CMN (sxtw/uxtw/sxtx/uxtx, amount 0..4 including bounds) matches llvm-mc (1000 cases).
- Negative immediate `cmn Rn, #-N` for N in 1..4095 matches llvm-mc's gas rewrite to `cmp Rn, #N` (1000 cases).
- encode_cmn(ops) equals encode_add_sub([ZR] ++ ops, is_sub=false, set_flags=true) where ZR is WZR if Rn is 32-bit else XZR (1000 cases).
- Success-path immediate word: Rd=31, S=1, op=0, bits[28:24]=0b10001, sf from Rn width, unshifted imm12, Rn at [9:5].
- 0 or 1 operands always Err.
- Non-register first operand (Imm/Symbol/Mem/Cond/Shift) with a second operand always Err.
- Known-answer: `cmn x0, #42` encodes as 0xb100a81f; `cmn w0, #42` as 0x3100a81f; `cmn x0, x1` as 0xab01001f; `cmn sp, #0` as 0xb10003ff; `cmn x0, w1, sxtw` as 0xab21c01f; `cmn x0, #-1` as 0xf100041f (`cmp x0, #1`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CMN immediate-form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `cmn xzr, #0`).
- CMN shifted-register Rm=31 is XZR/WZR; SP as Rm without extend is rejected by llvm-mc.
- Known-answer: `adds xzr, x0, #42` disassembles as `cmn x0, #42` with the same encoding.

## Quirks

- Extra operands beyond a Shift/Extend are ignored (see bugs).
- encode_cmn does not reject XZR/WZR as immediate-form Rn, so `cmn xzr, #0` encodes as `cmn sp, #0` (see bugs).
- sf is taken from the prepended ZR; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp to 31, so `cmn x0, sp` encodes as `cmn x0, xzr` (see bugs).
- encode_add_sub negates a negative Imm with `-imm_signed`, which panics on i64::MIN in debug (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (non-Reg first operand, extended-register form, negative-imm rewrite).

---

# Confirmed invariants (encode_cinv)

- Same-width GPR CINV (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cinv([Rd, Rn, cond]) equals encode_csinv([Rd, Rn, Rn, invert(cond)]) (ARM ARM CINV alias of CSINV) (1000 cases).
- encode_cinv([Rd, ZR, cond]) equals encode_csetm([Rd, cond]) (CSETM is CINV with Rn=XZR/WZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=Rn at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=00 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the three slots always Err.
- Known-answer: `cinv x0, x1, eq` encodes as 0xda811020; `cinv w0, w1, ne` as 0x5a810020; `cinv x0, xzr, eq` as 0xda9f13e0 (same as `csetm x0, eq`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CINV register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cinv sp, ...`).
- CINV takes Wt/Xt only (llvm-mc rejects `cinv d0, ...`).
- Cond AL and NV are invalid for the CINV alias (llvm-mc: "condition codes AL and NV are invalid for this instruction").
- llvm-mc rejects mixed x/w and a fourth operand.
- llvm-mc disassembles `cinv x0, xzr, eq` as `csetm x0, eq` with the same encoding.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- encode_cond accepts al/nv, so CINV AL/NV encodes as CSINV with inverted cond (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cinv sp, ...` encodes as `cinv xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None, get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cinc)

- Same-width GPR CINC (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cinc([Rd, Rn, cond]) equals encode_csinc([Rd, Rn, Rn, invert(cond)]) (ARM ARM CINC alias of CSINC) (1000 cases).
- encode_cinc([Rd, ZR, cond]) equals encode_cset([Rd, cond]) (CSET is CINC with Rn=XZR/WZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=Rn at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the three slots always Err.
- Known-answer: `cinc x0, x1, eq` encodes as 0x9a811420; `cinc w0, w1, ne` as 0x1a810420; `cinc x0, xzr, eq` as 0x9a9f17e0 (same as `cset x0, eq`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CINC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cinc sp, ...`).
- CINC takes Wt/Xt only (llvm-mc rejects `cinc d0, ...`).
- Cond AL and NV are invalid for the CINC alias (llvm-mc: "condition codes AL and NV are invalid for this instruction").
- llvm-mc rejects mixed x/w and a fourth operand.
- llvm-mc disassembles `cinc x0, xzr, eq` as `cset x0, eq` with the same encoding.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- encode_cond accepts al/nv, so CINC AL/NV encodes as CSINC with inverted cond (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cinc sp, ...` encodes as `cinc xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None, get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_adc)

- Same-width GPR ADC/ADCS (x0–x30/xzr and w0–w30/wzr, both S values) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_adc(ops, true) XOR encode_adc(ops, false) = 1<<29 (ARM ARM S bit).
- Success-path word: Rd at [4:0], Rn at [9:5], Rm at [20:16], sf at 31, S at 29, op at 30 = 0, bits [28:21] = 0b11010000, bits [15:10] = 0.
- Fewer than 3 operands always Err.
- Non-register (Imm/Mem/Shift/Symbol/Cond) in any of the three slots always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1) always Err.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `adc sp, ...`).
- Known-answer: `adc x0, x1, x2` encodes as 0x9a020020.

## Quirks

- encode_adc does not inspect operands beyond index 2, so a trailing Shift is silently dropped (see bugs).
- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_add_sub)

- Immediate-form ADD/SUB/ADDS/SUBS with a valid imm12 or auto-shift (N<<12, N in 1..=0xFFF), including negative-imm alias, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Shifted-register form (LSL/LSR/ASR, Rd/Rn not SP, amount in range) matches llvm-mc.
- NEON vector ADD/SUB Vd.T, Vn.T, Vm.T for T in {8b,16b,4h,8h,2s,4s,2d} matches llvm-mc.
- Fewer than 3 operands always returns Err containing "requires 3 operands".
- encode_add_sub([Rd,Rn,Imm(-N)], is_sub, s) equals encode_add_sub([Rd,Rn,Imm(N)], !is_sub, s) for valid positive N.
- :lo12: Modifier and ModifierOffset produce WordWithReloc { AddAbsLo12, symbol, addend } with imm12 field 0 and ADD-immediate opcode bits (1000 cases).
- :tprel_lo12_nc: / :tprel_hi12: produce TlsLeAddTprelLo12 / TlsLeAddTprelHi12 with sh bit 0 / 1 (1000 cases).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Immediate form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `add Rd, XZR, #imm`).
- ADDS/SUBS Rd cannot be SP/WSP (llvm-mc rejects `adds sp, ...`).
- Known-answer: `add x0, x1, #42` encodes as 0x9100a820.

## Quirks

- llvm-mc may disassemble `add w0, wsp, #0` as `mov w0, wsp`; the encoding word still matches.
- llvm-mc may rewrite `add x0, x1, #4096, lsl #0` as `add x0, x1, #1, lsl #12`.
- proptest `prop_assert_eq!` format strings cannot use implicit captures (`{asm}`); use `{}` + args.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (FP regs, ADDS Rd=SP, tprel modifiers).
- explicit_shift is true only for lsl#12; other immediate-form shifts are ignored (see bugs).
- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- ADDS/SUBS with Rd=SP encodes register 31 as XZR (see bugs).

---

# Confirmed invariants (IrConst::cast_float_to_target)

- F64 identity: `cast_float_to_target(fv, F64)` is `Some(F64(fv))` with bit-identical payload (NaN payload and signed zero preserved); 1000 random bit patterns.
- Signed in-range truncation toward zero: for I8/I16/I32/I64, the integer payload equals trunc_toward_zero(fv) when that integer is in range (seed: 3.125 → I32(3)).
- IrType::Void always returns None.
- U8 values in 128..=255 are not saturated to i8::MAX (127); the 8-bit pattern equals n as u8. (Storage form is still I8, so to_i64() sign-extends — see bugs.)
- F32 preserves sign of finite-nonzero and infinite inputs; infinities stay infinite.
- Ptr agrees with from_i64(n, Ptr) / ptr_int for in-range exact integers (default LP64 → I64).

## Environment

- Default target_ptr_size is 8 (LP64). IrConst does not implement PartialEq; tests compare via variant match / to_bits().
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit.

## Quirks

- `from_i64` stores U8/U16/U32 as I64; `cast_float_to_target` stores U8 as I8 and U16 as I16 (U32 already I64). `zero()`/`one()` also use I8 for U8.
- F128 arm calls `long_double` → `f64_to_f128_bytes_lossless`, which panics on f64 subnormals (biased_exp=0, mantissa≠0) via `u128` subtraction underflow.

---

# Confirmed invariants (classify_cast_with_f128)

- Identity: classify(ty, ty, native) = Noop for every IrType and both native flags (1000 cases).
- Non-native F128 reduction: classify(from, to, false) = classify(F128↦F64(from), F128↦F64(to), false).
- native flag is a no-op when neither endpoint is F128.
- Native F32/F64 ↔ F128 is FloatToF128 / F128ToFloat with the from_f32 / to_f32 flag.
- Integer-to-integer casts match size/signedness (IntWiden / IntNarrow / SignedToUnsignedSameSize / UnsignedToSignedSameSize / Noop).
- F32→F64 is FloatToFloat { widen: true }; F64→F32 is FloatToFloat { widen: false }.
- f128_is_native=false never returns SignedToF128 / UnsignedToF128 / F128ToSigned / F128ToUnsigned / FloatToF128 / F128ToFloat.
- Ptr ↔ pointer-width integer is Noop (I32/U32 on ILP32, I64/U64 on LP64).

## Environment

- Default target_ptr_size is 8 (LP64). Tests that exercise ILP32 use set_target_ptr_size(4) with a Drop guard so the thread-local is restored.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit.

## Quirks

- Ptr normalization (Ptr ≡ U64/U32) is applied only when neither endpoint is float. Float/F128 ↔ Ptr skips it: Ptr→float is SignedToFloat / SignedToF128, and float→Ptr always sets to_u64=true. See bug_reports/classify_cast_ptr_not_normalized_for_float.md.

---

# Confirmed invariants (encode_adr)

- Immediate-form ADR with Xd (x0–x30/xzr) and imm in [-1048576, 1048575] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases, bounds forced).
- Success-path word: bit 31 (op) = 0, bits [28:24] = 0b10000, Rd at [4:0], SignExtend21(immlo[30:29] | immhi[23:5]<<2) = imm.
- Changing Rd does not change opcode/imm fields; changing imm does not change Rd.
- Symbol / Label / SymbolOffset produce WordWithReloc { AdrPrelLo21, symbol, addend } with word = 0x10000000|rd and imm fields 0 (1000 cases).
- Empty operands, Imm-only, Rd-only, Mem second operand, and invalid name x32 always Err.
- Parser-misclassified Reg/Cond/Barrier names at operand 1 are treated as symbols (get_symbol workaround) and emit AdrPrelLo21.
- Known-answer: `adr x0, #0` encodes as 0x10000000; `adr x0, #1` encodes as 0x30000000.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADR register 31 is XZR, never SP (llvm-mc rejects `adr sp, ...`).
- ADR takes Xd only (llvm-mc rejects `adr w0, ...` and `adr d0, ...`).
- 21-bit signed range: [-1048576, 1048575]; llvm-mc rejects #1048576 and #-1048577.

## Quirks

- encode_adr ignores the is_64 flag from get_reg, so W and FP names encode as Xd with the same register number (see bugs).
- parse_reg_num maps sp to 31, so `adr sp, #imm` encodes as `adr xzr, #imm` (see bugs).
- Out-of-range immediates are truncated to 21 bits via `imm as u32` (see bugs). TODO at load_store.rs:697 notes the missing check.
- get_symbol accepts Modifier / ModifierOffset, so `:lo12:` / `:got:` produce AdrPrelLo21 instead of Err (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier + ModifierOffset).

---

# Confirmed invariants (encode_bic)

- Same-width GPR BIC register form (x0–x30/xzr and w0–w30/wzr, optional lsl/lsr/asr/ror in range) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid BIC-immediate (inverted value is an AArch64 bitmask), including Rd=SP/WSP, matches llvm-mc (1000 cases). Encodes as AND with #~imm.
- NEON BIC Vd.T, Vn.T, Vm.T for T in {8b, 16b} matches llvm-mc (1000 cases).
- encode_bic([Rd, Rn, Imm(imm)]) equals encode_logical([Rd, Rn, Imm(~imm)], opc=00) for valid bitmasks (1000 cases).
- Fewer than 3 operands always Err.
- Operand 2 that is Mem/Symbol/Cond/Label/Barrier always Err.
- Invalid Rm names (x32, w32, empty, foo, r0, x) always Err.
- Immediates llvm-mc rejects as non-bitmasks (#0, all-ones, #5, #9, #0x11) are also rejected by encode_bic.
- Known-answer: `bic x0, x1, x2` encodes as 0x8a220020; `bic x0, x1, #1` as 0x927ff820; `bic v0.16b, v1.16b, v2.16b` as 0x4e621c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Register form: register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `bic sp, ...`).
- Immediate form: Rd of 31 is SP/WSP, not XZR (llvm-mc rejects `bic xzr, x0, #1`; accepts `bic sp, x0, #1`).
- NEON three-same T is 8B or 16B only.
- Shift amount: W-form [0, 31], X-form [0, 63]. Bound+1 (32 / 64) is rejected by llvm-mc.

## Quirks

- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp/wsp to 31, so register-form SP encodes as XZR (see bugs).
- Immediate-form XZR/WZR encodes as SP/WSP (see bugs).
- Shift amount is masked with 0x3F; 32-bit lsl #32 is accepted (see bugs).
- encode_neon_bic sets Q only for 16b; 8h/4s/2d encode as 8b (see bugs).
- Unknown shift kinds fall through to LSL (`_ => 0b00`) (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid bitmask, unsupported third operand, invalid rm, unknown shift kind).

---

# Confirmed invariants (encode_neon_three_diff_narrow)

- Valid ADDHN/RADDHN/SUBHN/RSUBHN (+2) with mandated (Ta,Tb) pairs matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Ta∈{8h,4s,2d}; Tb is 8b/16b, 4h/8h, 2s/4s according to is_high.
- encode(..., is_high=true) XOR encode(..., is_high=false) = 1<<30 (ARM ARM Q bit).
- encode(..., u=1) XOR encode(..., u=0) = 1<<29 (ARM ARM U bit).
- Success-path word: bit 31 = 0, bits [28:24] = 0b01110, bit 21 = 1, bits [11:10] = 00, Rd at [4:0], Rn at [9:5], Rm at [20:16], opcode at [15:12], size at [23:22] from Ta (8h=00, 4s=01, 2d=10).
- Fewer than 3 operands always Err.
- Unsupported source Ta (not 8h/4s/2d) always Err.
- Non-register (Imm/Mem/Symbol/Shift/Cond/Label) in any of the three slots always Err.
- Invalid NEON register names (v32, v99, foo, empty, v, v-1) always Err.
- Known-answer: `addhn v0.8b, v1.8h, v2.8h` encodes as 0x0e224020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADDHN2/RADDHN2/SUBHN2/RSUBHN2 set Q=1 (upper half).
- U=1 is the rounding form (RADDHN/RSUBHN); opcode 0b0100 add-family, 0b0110 sub-family.

## Quirks

- Dest arrangement Tb is ignored (see bugs).
- Rm arrangement is ignored; size comes only from operand 1 (see bugs).
- Extra operands beyond 3 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b, so GPR/FP dest encodes as Vd (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid register names via get_neon_reg).

---

# Confirmed invariants (encode_bics)

- Same-width GPR BICS register form (x0–x30/xzr and w0–w30/wzr, optional lsl/lsr/asr/ror in range) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_bics(ops) XOR encode_bic(ops) = 0b11<<29 for the same 3-reg (+ in-range shift) operands (ARM ARM opc field).
- Success-path word: sf at 31, opc=11 at [30:29], bits [28:24]=0b01010, N=1 at 21, Rd at [4:0], Rn at [9:5], Rm at [20:16], shift at [23:22], imm6 at [15:10].
- Fewer than 3 operands always Err.
- Invalid Rm names (x32, w32, empty, foo, r0, x) always Err.
- Known-answer: `bics x0, x1, x2` encodes as 0xea220020; `bics w0, w1, w2` as 0x6a220020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Register form: register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `bics sp, ...`).
- Immediate form: GNU/llvm-mc alias `bics Rd, Rn, #imm` → `ands Rd, Rn, #~imm` (Rd=XZR becomes TST). SP/WSP is not a valid Rd.
- Shift amount: W-form [0, 31], X-form [0, 63]. Bound+1 (32 / 64) is rejected by llvm-mc.
- BICS has no NEON form (llvm-mc rejects `bics v0.16b, ...`).

## Quirks

- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp/wsp to 31, so register-form SP encodes as XZR (see bugs).
- Shift amount is masked with 0x3F; 32-bit lsl #32 is accepted (see bugs).
- Unknown shift kinds fall through to LSL (`_ => 0b00`) (see bugs).
- Immediate form is not implemented; get_reg on operand 2 returns Err (see bugs).
- A 4th operand that is not Shift is ignored (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid rm names, extra 4th operand).

---

# Confirmed invariants (encode_bl)

- Symbol / Label produce WordWithReloc { Call26, symbol, addend: 0 } with word = 0x94000000 (bits[31:26]=100101, imm26=0) (1000 cases).
- SymbolOffset(s, addend) preserves symbol and addend and still uses Call26 / 0x94000000 (1000 cases).
- Call26.elf_type() = 283 (R_AARCH64_CALL26).
- encode_bl(ops) XOR encode_branch(ops) = 1<<31 for the same SymbolOffset operands; BL reloc is Call26 and B reloc is Jump26 (1000 cases).
- Empty operands always Err.
- Unaligned or out-of-range Imm (bound±1 / ±4, #1, i64::MIN/MAX) always Err.
- Parser-misclassified Reg/Cond/Barrier names produce Call26 with that name (get_symbol workaround).
- Known-answer (llvm-mc): `bl #0` encodes as 0x94000000; `bl #4` as 0x94000001; `bl #-134217728` as 0x96000000. SUT currently rejects Imm (see bugs).
- Known-answer (SUT): `bl foo` → WordWithReloc { 0x94000000, Call26, "foo", 0 }.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM BL range: aligned offsets in [-134217728, 134217724]; llvm-mc rejects #1, #134217728, #-134217732.
- llvm-mc `bl foo` emits R_AARCH64_CALL26 with instruction word 0x94000000 (imm26 filled later).

## Quirks

- encode_bl does not encode the immediate form; get_symbol rejects Imm (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- get_symbol accepts Modifier / ModifierOffset, dropping the kind, so `:lo12:` becomes Call26 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier).

---

# Confirmed invariants (encode_blr)

- `blr Xn` / `blr xzr` / `blr lr` (x0–x30, xzr, lr, uppercase X0) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: 0xd63f0000 | (rn << 5); bits[31:25]=1101011, opc[24:21]=0001, op4[4:0]=0, Rn at [9:5] (1000 cases).
- encode_blr(ops) XOR encode_br(ops) = 1<<21 for the same Xn operand (ARM ARM opc bit 21) (1000 cases).
- Empty operands always Err.
- Non-register (Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Known-answer: `blr x0` encodes as 0xd63f0000; `blr x17` as 0xd63f0220.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- BLR register 31 is XZR, never SP (llvm-mc rejects `blr sp`).
- BLR takes Xn only (llvm-mc rejects `blr w0` and `blr d0`).
- llvm-mc accepts `blr x31` as `blr xzr`; `blr lr` as `blr x30`.
- Codegen emits `blr x17` for indirect calls (calls.rs:233).

## Quirks

- encode_blr discards the is_64 flag from get_reg, so W names encode as Xn (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `blr sp` encodes as `blr xzr` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_reg (parse_reg_num None via encode_blr_neg_invalid_name).

---

# Confirmed invariants (encode_br)

- `br Xn` / `br xzr` / `br lr` (x0–x30, xzr, lr, uppercase X0) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: 0xd61f0000 | (rn << 5); bits[31:25]=1101011, opc[24:21]=0000, op4[4:0]=0, Rn at [9:5] (1000 cases).
- encode_blr(ops) XOR encode_br(ops) = 1<<21 for the same Xn operand (ARM ARM opc bit 21) (1000 cases).
- Empty operands always Err.
- Non-register (Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Known-answer: `br x0` encodes as 0xd61f0000; `br x17` as 0xd61f0220.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- BR register 31 is XZR, never SP (llvm-mc rejects `br sp`).
- BR takes Xn only (llvm-mc rejects `br w0` and `br d0`).
- llvm-mc accepts `br x31` as `br xzr`; `br lr` as `br x30`.
- Codegen emits `br x0` for indirect jumps (emit.rs:1760) and `br x17` for jump tables (emit.rs:1808).

## Quirks

- encode_br discards the is_64 flag from get_reg, so W names encode as Xn (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `br sp` encodes as `br xzr` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_reg (parse_reg_num None via encode_br_neg_invalid_name).

---

# Confirmed invariants (encode_branch)

- Symbol / Label / SymbolOffset produce WordWithReloc { Jump26, symbol, addend } with word = 0x14000000 and imm26 field 0 (1000 cases). ELF type is 282 (R_AARCH64_JUMP26).
- encode_bl(ops).word XOR encode_branch(ops).word = 1<<31 for the same SymbolOffset operands (ARM ARM bit 31); reloc types Call26 vs Jump26; same symbol and addend (1000 cases).
- Success-path reloc word: bits[31:26] = 000101, bits[25:0] = 0.
- Empty operands always Err.
- Unaligned or out-of-range Imm always Err (because all Imm currently Err — see bugs).
- Parser-misclassified Reg/Cond/Barrier names at operand 0 are treated as symbols (get_symbol workaround) and emit Jump26 (1000 cases).
- Known-answer: llvm-mc `b #0` encodes as 0x14000000; `b #4` as 0x14000001. SUT does not yet match (see bugs).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM B signed PC offset: [-134217728, 134217724], multiple of 4.
- llvm-mc rejects bare `b` (too few operands), `b foo, x0` (invalid operand), `b #1` (expected label or encodable integer pc offset), `b :lo12:foo`.
- Codegen emits `b <label>`, not `b #imm`.

## Quirks

- encode_branch never encodes Imm: get_symbol rejects it (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- get_symbol accepts Modifier / ModifierOffset, so `:lo12:` produces Jump26 instead of Err (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier via encode_branch_symbol_misclassified).

---

# Confirmed invariants (encode_cbz)

- Symbol / Label / SymbolOffset produce WordWithReloc { CondBr19, symbol, addend } with word = (sf<<31)|(0b011010<<25)|(op<<24)|Rt and imm19 field 0 (1000 cases). ELF type is 280 (R_AARCH64_CONDBR19).
- encode_cbz(ops, true).word XOR encode_cbz(ops, false).word = 1<<24 for the same SymbolOffset operands (ARM ARM op bit); both reloc types CondBr19; same symbol and addend (1000 cases).
- Success-path reloc word: bits[30:25] = 011010, sf at 31 from Rt width, op at 24 from is_nz, Rt at [4:0], bits[23:5] = 0.
- Empty operands and a missing label always Err.
- Unaligned or out-of-range Imm always Err (because all Imm currently Err — see bugs).
- Mem / Shift / Extend / RegArrangement / Expr / RegList in the label slot always Err.
- Parser-misclassified Reg/Cond/Barrier names at operand 1 are treated as symbols (get_symbol workaround) and emit CondBr19 (1000 cases).
- Known-answer: llvm-mc `cbz x0, #0` encodes as 0xb4000000; `cbz w0, #0` as 0x34000000; `cbnz x0, #4` as 0xb5000020. SUT does not yet match (see bugs). llvm-mc `cbz x0, foo` is a CondBr19 reloc with word 0xb4000000 — SUT matches.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM CBZ/CBNZ signed PC offset: [-1048576, 1048572], multiple of 4.
- CBZ register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cbz sp, ...`).
- CBZ takes Wt/Xt only (llvm-mc rejects `cbz d0, ...` and `cbz wsp, ...`).
- llvm-mc rejects bare `cbz` / `cbz x0` (too few operands), `cbz x0, #0, x1` (invalid operand), `cbz x0, #1` (expected label or encodable integer pc offset).
- Codegen emits `cbz xN, .Llabel` / `cbnz wN, .Llabel`, not `cbz Rt, #imm`.

## Quirks

- encode_cbz never encodes Imm: get_symbol rejects it (see bugs).
- Extra operands beyond index 1 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cbz sp, L` encodes as `cbz xzr, L` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- llvm-mc accepted `cbz x0, :lo12:foo` as a branch19 fixup; get_symbol also accepts Modifier (kind discarded).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier via encode_cbz_symbol_misclassified; other kinds via encode_cbz_neg_bad_label_kind).

---

# Confirmed invariants (encode_ccmp_ccmn)

- Same-width GPR CCMP/CCMN immediate form (`Rn, #imm5, #nzcv, cond` with imm5 in [0,31], nzcv in [0,15], all 16 cond codes plus hs/cs/lo/cc aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-width GPR CCMP/CCMN register form (`Rn, Rm, #nzcv, cond`) matches llvm-mc (1000 cases).
- encode_ccmp_ccmn(ops, true).word XOR encode_ccmp_ccmn(ops, false).word = 1<<30 (ARM ARM op bit) for both forms (1000 cases).
- Success-path word: sf at 31, op at 30, S=1 at 29, bits [28:21]=0b11010010, cond at [15:12], Rn at [9:5], nzcv at [3:0], bit 10=0, bit 4=0; o2 at 11 is 1 for immediate (imm5 at [20:16]) and 0 for register (Rm at [20:16]).
- Fewer than 4 operands always Err.
- Invalid condition names (xx, foo, empty, eqz, n, zzzz) always Err.
- Invalid Rm names (x32, w32, foo, empty, r0, x, x-1, x99) always Err.
- Mem / Symbol / Shift / Extend / Label / Barrier in slots 1, 2, or 3 always Err.
- Known-answer: `ccmp x0, #0, #0, eq` encodes as 0xfa400800; `ccmp x0, x1, #0, eq` as 0xfa410000; `ccmn x0, #0, #0, eq` as 0xba400800.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CCMP/CCMN register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ccmp sp, ...` and `ccmp wsp, ...`).
- CCMP/CCMN takes Wt/Xt only (llvm-mc rejects `ccmp d0, ...`).
- imm5 unsigned [0, 31]; nzcv unsigned [0, 15]; llvm-mc rejects #-1, #32, #16.
- llvm-mc rejects mixed x/w (`ccmp x0, w1` / `ccmn w0, x0`) and a fifth operand.

## Quirks

- imm5 is stored as `*imm5 as u32 & 0x1F` and nzcv as `*nzcv as u32 & 0xF`, so out-of-range values are truncated (see bugs).
- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `ccmp sp, ...` encodes as `ccmp xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rm width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None on Rm, unsupported operand kinds).

---

# Confirmed invariants (encode_neon_shll)

- Valid SSHLL/USHLL(+2) with mandated (Tb,Ta) and shift in [0, esize-1] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- is_high XOR toggles only Q (bit 30) (1000 cases).
- u_bit XOR toggles only U (bit 29) (1000 cases).
- encode_neon_shll(..., Imm(0)) equals encode_neon_xtl and llvm-mc sxtl/uxtl(+2) (1000 cases).
- Success-path word: bit31=0, Q at 30, U at 29, bits[28:23]=011110, immh:immb=esize+shift, opcode=101001, Rn, Rd.
- Arity < 3, unsupported source Tb, non-matching operand kinds, and invalid NEON names always Err (1000 cases).
- Known-answer: `sshll v0.8h, v1.8b, #0` = 0x0f08a420; `#7` = 0x0f0fa420; `ushll` #0 = 0x2f08a420; `sshll2 v0.8h, v1.16b, #0` = 0x4f08a420; `ushll2 ... #7` = 0x6f0fa420; `sshll v0.4s, v1.4h, #0` = 0x0f10a420; `#15` = 0x0f1fa420; `sshll v0.2d, v1.2s, #0` = 0x0f20a420; `#31` = 0x0f3fa420; `ushll2 v0.2d, v1.4s, #31` = 0x6f3fa420; `sxtl v0.8h, v1.8b` = 0x0f08a420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD SSHLL/USHLL: `0 Q U 011110 immh immb 101001 Rn Rd`. Q=0 Tb={8B,4H,2S}; Q=1 Tb={16B,8H,4S}. Ta is 8H/4S/2D. shift in 0..(esize-1). immh:immb = esize + shift.
- Dispatch: encoder/mod.rs:614-617 ushll/ushll2/sshll/sshll2. Sibling encode_neon_xtl is the documented #0 alias (same job at shift 0).
- Callers: assembler README NEON widen/long table lists sshll/ushll/sxtl/uxtl (+2).

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Dest arrangement is discarded (see bugs).
- Shift is `get_imm as u32` with no range check; #8 for 8b encodes as 16-bit esize; #-1 overflows in debug (see bugs).
- Operand::Reg dest (GPR/FP names) encodes via parse_reg_num (see bugs).
- Q comes only from is_high, not from Tb, so sshll2+8b and sshll+16b encode (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity < 3, unsupported Tb, kinds, invalid names, GPR dest, Q vs Tb).

---

# Confirmed invariants (encode_neon_sqshrun)

- Valid vector SQSHRUN/SQRSHRUN (+2) with Ta in {8h,4s,2d}, Tb matching Ta and the 2-suffix, Vd/Vn in v0–v31, shift in [1, dest_esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., is_high=false) XOR encode(..., is_high=true) = 1<<30 (ARM ARM Q bit) (1000 cases).
- encode(..., is_rounding=false) XOR encode(..., is_rounding=true) = 1<<11 (opcode 100001 vs 100011) (1000 cases).
- Success-path word: bit 31=0, Q at 30, U=1 at 29, bits [28:23]=011110, immh:immb at [22:16]=src_esize-shift, opcode at [15:10]=100001/100011, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, unsupported source Ta (not 8h/4s/2d), non-RegArrangement/non-Imm kinds, invalid names (v32, foo, empty, v, v-1), and bare Operand::Reg source always Err.
- Known-answer: `sqshrun v0.8b, v1.8h, #1` encodes as 0x2f0f8420; `#8` as 0x2f088420; `sqshrun2 v0.16b, v1.8h, #1` as 0x6f0f8420; `sqrshrun v0.8b, v1.8h, #1` as 0x2f0f8c20; `sqrshrun2 v0.4s, v1.2d, #32` as 0x6f208c20; `sqshrun v0.4h, v1.4s, #16` as 0x2f108420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SQSHRUN/SQRSHRUN: `0 Q 1 011110 immh immb opcode Rn Rd`. opcode 100001 non-rounding / 100011 rounding. U=1 always (signed-to-unsigned saturating narrow). Q=0 lower half / Q=1 (`2` suffix) upper half. dest_esize = 8<<HighestSetBit(immh); shift = 2*esize - UInt(immh:immb) in [1, dest_esize]. Ta/Tb: 8H→8B/16B (1..8), 4S→4H/8H (1..16), 2D→2S/4S (1..32).
- Dispatch: encoder/mod.rs:649-650 sqrshrun/sqrshrun2; encoder/mod.rs:841-842 sqshrun/sqshrun2.
- Sibling encode_neon_shrn neon.rs:1443-1444 checks `shift > half_bits` with half_bits = source/2. encode_neon_qshrn / encode_neon_scalar_qshrn fail the same-job gate.
- Callers: assembler README NEON narrow table; no codegen emission of sqshrun found.

## Quirks

- Shift range uses source element size (16/32/64), so dest_esize+1 through source_esize encode (see bugs).
- Dest arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg, so GPR/FP dest encodes as Vd (see bugs).
- Shift is `*v as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- Bare Operand::Reg source Errs via empty arrangement (not a bug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Ta / *v as u32 / get_neon_reg Reg dest+source / extra / mismatched Tb).

---

# Confirmed invariants (encode_neon_shift_left_imm)

- Valid vector SQSHL/UQSHL immediate with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [0, esize-1] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29 (ARM ARM U bit SQSHL vs UQSHL) (1000 cases).
- Same-esize Q=0 vs Q=1 arrangements XOR = 1<<30 (1000 cases).
- shift+1 (still in range) adds 1 to immh:immb at bits [22:16] (1000 cases).
- Success-path word: bit 31=0, Q at 30, U at 29, bits [28:23]=011110, immh:immb at [22:16]=esize+shift, opcode at [15:11]=01110, bit 10=1, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, unsupported T (1d, 8s, empty), invalid names (v32, foo, empty, v, v-1), non-register kinds, and non-Imm shift always Err.
- Known-answer: `sqshl v0.8b, v1.8b, #0` encodes as 0x0f087420; `#7` as 0x0f0f7420; `uqshl v0.8b, v1.8b, #0` as 0x2f087420; `sqshl v0.16b, v1.16b, #3` as 0x4f0b7420; `sqshl v0.4h, v1.4h, #15` as 0x0f1f7420; `sqshl v0.2d, v1.2d, #0` as 0x4f407420; `#63` as 0x4f7f7420; `uqshl v31.4s, v30.4s, #31` as 0x6f3f77df.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift left (immediate) SQSHL/UQSHL: `0 Q U 011110 immh immb 01110 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 is Reserved (no 1D). shift = UInt(immh:immb) - esize in [0, esize-1]. U=0 SQSHL / U=1 UQSHL. opcode=01110.
- Dispatch: encoder/mod.rs:544-551 sqshl Imm => u=0; uqshl Imm => u=1. Register-form sqshl/uqshl go to encode_neon_three_same (different job).
- Sibling encode_neon_shl is SHL (opcode 01010, U=0). Sibling encode_neon_sli is SLI (opcode 01010, U=1). Same-job gate fails.
- Callers: assembler README NEON shifts table lists sqshl/uqshl. Scalar SQSHL Bd/Hd/Sd/Dd uses a different encoding (out of scope).

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Negative Imm debug-panics (`esize + (shift as u32)` overflow); shift == esize wraps into the next lane size; i64 Imm truncates via `as u32` (see bugs).
- parse_reg_num accepts x/w/d/s/q/h/b prefixes, so non-V names encode as V registers (see bugs).
- get_neon_reg accepts Operand::Reg, so a bare V/GPR source encodes as Rn (see bugs). Dest as Operand::Reg still Errs via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / T / extra / shift range / mismatched T / non-V prefix / Operand::Reg dest+source).

