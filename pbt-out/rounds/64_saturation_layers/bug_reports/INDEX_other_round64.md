# INDEX — Round-64 independents (TLS/ELF/C23/asm/diagnostics/driver)

Full bodies in per-bug .md files in this directory and in the linked issues.

| # | ledger name |
|---|---|
| [#1589](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1589) | unnamed bitfield inflates struct sizeof (8v5, 4v3) |
| [#1590](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1590) | _Static_assert sizeof(struct/union/enum) vacuous |
| [#1591](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1591) | driver -u misparsed as input file |
| [#1592](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1592) | UCN mid-identifier rejected (leading works) |
| [#1593](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1593) | $ identifier → bogus assembler error |
| [#1594](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1594) | asm-label rename dropped on definitions; vars "bad register" |
| [#1600](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1600) | types_compatible_p ignores pointee qualifiers |
| [#1601](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1601) | types_compatible_p void(void) parse error |
| [#1602](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1602) | ARM+RV lack .equiv/.sleb128/.uleb128 (x86 has) |
| [#1603](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1603) | __PIC__ never defined (gcc: always) |
| [#1605](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1605) | __STDC_NO_ATOMICS__ defined as 0 (gcc: undefined) |
| [#1606](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1606) | long-double literals 1 ULP low (truncation) — 6/12 sampled |
| [#1607](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1607) | object/typedef redeclaration compat absent (3 forms) |
| [#1608](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1608) | initializer type constraints absent (4 forms) |
| [#1609](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1609) | bit-field base-type constraints absent (7 forms) |
| [#1610](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1610) | ## in object-like macro not expanded |
| [#1611](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1611) | __typeof__(VLA) decays to pointer (4 vs 12) |
| [#1612](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1612) | hex-float subnormal literals → +0.0 (decimal path OK) |
| [#1613](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1613) | wide-prefix string concatenation L+u accepted |
| [#1615](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1615) | 5 driver query flags rejected (-print-prog-name/--help/...) |
| [#1616](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1616) | __DATE__/__TIME__ hardcoded (Jan 1 2025 / 00:00:00) |
| [#1617](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1617) | enum value not required to be ICE (incl. 1/0) |
| [#1618](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1618) | duplicate function parameter names accepted |
| [#1619](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1619) | x86 multi-byte NOP mem forms (0F 1F) all rejected |
| [#1620](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1620) | undefined numeric local label (1f) → silent disp-0 jump |
| [#1621](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1621) | cvtss2si/cvtsd2si to %eax spurious REX.W (wrong width) |
| [#1622](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1622) | AT&T cbtw/cwtl/cwtd sign-extension mnemonics missing |
| [#1623](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1623) | %cs:/%ds:/%ss: segment prefixes rejected (fs/gs only) |
| [#1624](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1624) | MMX reg-count shifts (8) + movdq2q/movq2dq rejected |
| [#1625](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1625) | 16 AVX2 vpmovzx/sx + vcvt float-family forms rejected |
| [#1626](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1626) | btsw drops 0x66 (16-bit BTS silently 32-bit) + lock bts reg,reg accepted |
| [#1627](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1627) | verr %ax missing (verw facet = #1356) |
| [#1628](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1628) | fsub/fdiv %st,%st(1) encode REVERSED opcode (dce9/dcf9) — HIGH |
| [#1629](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1629) | movw seg-load drops 0x66 — DUP of #1396, closed |
| [#1630](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1630) | .eqv/.equiv→0 + .irpc single-0x00 (wrong-value directives) |
| [#1631](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1631) | .macro expansion before explicit section → no active section |
| [#1632](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1632) | AArch64 scalar fcmp/fcmpe #0.0 explicit → NOP (stale flags) |
| [#1633](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1633) | AArch64 16 LSE min/max atomics (ldsmax/ldsmin/ldumax/ldumin × 4) all missing |
| [#1634](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1634) | AArch64 ldsmax (LDSMAX — signed atomic MAX, relaxed (no ordering)) rejected |
| [#1635](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1635) | AArch64 ldsmaxa (LDSMAXA — signed atomic MAX, acquire) rejected |
| [#1636](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1636) | AArch64 ldsmaxl (LDSMAXL — signed atomic MAX, release) rejected |
| [#1637](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1637) | AArch64 ldsmaxal (LDSMAXAL — signed atomic MAX, acquire+release) rejected |
| [#1638](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1638) | AArch64 ldsmin (LDSMIN — signed atomic MIN, relaxed) rejected |
| [#1639](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1639) | AArch64 ldsmina (LDSMINA — signed atomic MIN, acquire) rejected |
| [#1640](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1640) | AArch64 ldsminl (LDSMINL — signed atomic MIN, release) rejected |
| [#1641](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1641) | AArch64 ldsminal (LDSMINAL — signed atomic MIN, acquire+release) rejected |
| [#1642](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1642) | AArch64 ldumax (LDUMAX — unsigned atomic MAX, relaxed) rejected |
| [#1643](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1643) | AArch64 ldumaxa (LDUMAXA — unsigned atomic MAX, acquire) rejected |
| [#1644](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1644) | AArch64 ldumaxl (LDUMAXL — unsigned atomic MAX, release) rejected |
| [#1645](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1645) | AArch64 ldumaxal (LDUMAXAL — unsigned atomic MAX, acquire+release) rejected |
| [#1646](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1646) | AArch64 ldumin (LDUMIN — unsigned atomic MIN, relaxed) rejected |
| [#1647](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1647) | AArch64 ldumina (LDUMINA — unsigned atomic MIN, acquire) rejected |
| [#1648](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1648) | AArch64 lduminl (LDUMINL — unsigned atomic MIN, release) rejected |
| [#1649](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1649) | AArch64 lduminal (LDUMINAL — unsigned atomic MIN, acquire+release) rejected |
| [#1650](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1650) | NEON sdot/udot/usdot dot-product family missing |
| [#1651](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1651) | FCMLA/FCADD complex arithmetic missing |
| [#1652](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1652) | FP16 vector arithmetic (fadd/fmul/fmla v.8h) missing |
| [#1653](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1653) | fcvtn2 .4h reserved arrangement accepted (SIGILL bytes) |
| [#1654](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1654) | AArch64 setf8 (SETF8 — set NZCV from byte-8 value in Wn) rejected |
| [#1655](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1655) | AArch64 setf16 (SETF16 — set NZCV from halfword value in Wn) rejected |
| [#1656](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1656) | AArch64 cfinv (CFINV — invert PSTATE.C (carry flag)) rejected |
| [#1657](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1657) | AArch64 axflag (AXFLAG — convert NZCV to alternative flag order) rejected |
| [#1658](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1658) | AArch64 xaflag (XAFLAG — restore NZCV from alternative order) rejected |
| [#1659](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1659) | AArch64 esb (ESB — Error Synchronization Barrier (RAS)) rejected |
| [#1660](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1660) | i686 lcall $sel,$off (far call) rejected |
| [#1661](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1661) | i686 DR0-DR7 debug-register MOVs rejected |
| [#1662](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1662) | x86-64 gf2p8affineqb rejected (GFNI/VAES gap) |
| [#1663](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1663) | x86-64 gf2p8affineinvqb rejected (GFNI/VAES gap) |
| [#1664](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1664) | x86-64 gf2p8mulb rejected (GFNI/VAES gap) |
| [#1665](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1665) | x86-64 vgf2p8affineqb rejected (GFNI/VAES gap) |
| [#1666](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1666) | x86-64 vaeskeygenassist rejected (GFNI/VAES gap) |
| [#1667](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1667) | x86-64 vaesimc rejected (GFNI/VAES gap) |
| [#1668](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1668) | AArch64 sshll source-arrangement mismatch accepted |
| [#1669](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1669) | x86 XOP: XOP vfrcz family (fraction-extract) missing |
| [#1670](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1670) | x86 XOP: XOP vpcmov (per-variable bit select) missing |
| [#1671](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1671) | x86 XOP: XOP integer multiply-accumulate vpmacs*/vpmacss* missing |
| [#1672](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1672) | x86 XOP: XOP byte-permute vpperm/vpermil2ps/vpermil2pd missing |
| [#1673](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1673) | x86 XOP: XOP horizontal add/sub vphaddw/vphaddubq/vphsubw missing |
| [#1674](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1674) | x86 XOP: XOP rotate/shift-by-register vprotb/vprotd/vpshlb missing |
| [#1675](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1675) | i686 aam (AAM) rejected |
| [#1676](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1676) | i686 aad (AAD) rejected |
| [#1677](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1677) | i686 daa (DAA) rejected |
| [#1678](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1678) | i686 das (DAS) rejected |
| [#1679](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1679) | i686 salc (SALC) rejected |
| [#1680](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1680) | x86 monitorx/mwaitx (AMD) rejected |
| [#1681](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1681) | RISC-V Zacas amocas rejected |
| [#1682](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1682) | x86 LOCK/REP prefix validation absent (7 illegal combos accepted) |
| [#1683](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1683) | x86 loop family half-missing + i686 jcxz drops 0x67 |
| [#1684](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1684) | x86 bnd jmp (F2 branch prefix) rejected |
| [#1685](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1685) | AArch64 sysl (system read form) rejected |
| [#1686](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1686) | RISC-V sinval.vma (Svinval) rejected |
| [#1687](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1687) | RISC-V CSR medeleg rejected |
| [#1688](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1688) | RISC-V CSR mideleg rejected |
| [#1689](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1689) | RISC-V CSR menvcfg rejected |
| [#1690](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1690) | RISC-V CSR senvcfg rejected |
| [#1691](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1691) | _Float16/__fp16/f16 suffix unrecognized (both backends, ARM-native type) |
| [#1692](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1692) | TLS init zero + cross-module corruption (.tdata %16=8) |
| [#1693](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1693) | shared-lib TLS relocs dropped (dlopen 0) |
| [#1694](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1694) | @tlsgd/@tlsld -> PC32; indntpoff accepted |
| [#1695](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1695) | PT_GNU_RELRO missing (GOT writable) |
| [#1696](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1696) | backtrace 1-frame: FDEs have no CFA rules |
| [#1697](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1697) | -Wl,-z,now silent no-op |
| [#1698](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1698) | asm goto fallthrough jumps to last label (skips next stmt) |
| [#1699](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1699) | inline-asm %= / Nf/Nb labels not expanded |
| [#1700](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1700) | inline-asm "x" constraint input not materialized |
| [#1701](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1701) | movd/movq GPR<->MMX rejected (0F 6E/7E) |
| [#1702](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1702) | C23 auto silently = int (miscompile) |
| [#1703](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1703) | C23 _BitInt absent |
| [#1704](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1704) | C23 constexpr absent |
| [#1705](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1705) | C23 [[attributes]] absent |
| [#1706](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1706) | C23 typeof_unqual absent |
| [#1707](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1707) | C23 nullptr/nullptr_t absent |
| [#1708](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1708) | C23 unreachable() absent |
| [#1709](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1709) | C23 digit separator ' absent |
| [#1710](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1710) | #elifdef silent wrong branch + __has_c_attribute false |
| [#1711](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1711) | #embed silently zero bytes |
| [#1712](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1712) | __FILE_NAME__ macro absent |
| [#1767](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1767) | tls_model attr ignored (GOTTPOFF vs TPOFF32) |
| [#1768](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1768) | deprecated/warn_unused_result/format/nonnull no warnings |
| [#1787](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1787) | C95 digraphs unsupported |
| [#1788](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1788) | u'' literal type int (4 not 2) |
| [#1789](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1789) | bitfield width exceeds type accepted |
| [#1790](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1790) | C23 enum fixed underlying type rejected |
| [#1791](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1791) | default warnings absent: implicit-int/missing-return/unknown-escape |
| [#1792](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1792) | .intel_syntax accepted but not implemented |
