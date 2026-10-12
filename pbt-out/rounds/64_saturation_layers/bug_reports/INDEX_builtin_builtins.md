# INDEX — __builtin_ dispatch gaps (link-level)

Witness per issue: gcc/clang `-lm` link OK; ccc `undefined symbol __builtin_X`; raw-symbol passthrough (`U __builtin_X`). 3 dups closed (#1784-#1786 -> #1596).

| # | ledger name |
|---|---|
| [#1595](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1595) | __builtin_object_size always (size_t)-1 |
| [#1596](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1596) | __builtin_apply_args/__builtin_return missing |
| [#1597](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1597) | __builtin_fma/fmaf missing |
| [#1598](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1598) | builtins wave2: powi/bcopy/index/rindex missing |
| [#1599](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1599) | builtins wave3: memchr/stpcpy/sincos/cpu_is/eh 5 more |
| [#1604](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1604) | __builtin_speculation_safe_* missing |
| [#1614](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1614) | 17 FP math builtins absent (sqrtl-family + truncf/fminf/fmaxf) |
| [#1713](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1713) | __builtin_exp2 missing (link-level) |
| [#1714](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1714) | __builtin_expm1 missing (link-level) |
| [#1715](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1715) | __builtin_log1p missing (link-level) |
| [#1716](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1716) | __builtin_cbrt missing (link-level) |
| [#1717](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1717) | __builtin_sinh missing (link-level) |
| [#1718](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1718) | __builtin_cosh missing (link-level) |
| [#1719](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1719) | __builtin_tanh missing (link-level) |
| [#1720](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1720) | __builtin_asinh missing (link-level) |
| [#1721](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1721) | __builtin_acosh missing (link-level) |
| [#1722](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1722) | __builtin_atanh missing (link-level) |
| [#1723](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1723) | __builtin_asin missing (link-level) |
| [#1724](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1724) | __builtin_acos missing (link-level) |
| [#1725](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1725) | __builtin_atan missing (link-level) |
| [#1726](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1726) | __builtin_tgamma missing (link-level) |
| [#1727](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1727) | __builtin_lgamma missing (link-level) |
| [#1728](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1728) | __builtin_erf missing (link-level) |
| [#1729](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1729) | __builtin_erfc missing (link-level) |
| [#1730](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1730) | __builtin_logb missing (link-level) |
| [#1731](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1731) | __builtin_trunc missing (link-level) |
| [#1732](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1732) | __builtin_nearbyint missing (link-level) |
| [#1733](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1733) | __builtin_rint missing (link-level) |
| [#1734](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1734) | __builtin_fmod missing (link-level) |
| [#1735](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1735) | __builtin_fdim missing (link-level) |
| [#1736](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1736) | __builtin_atan2 missing (link-level) |
| [#1737](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1737) | __builtin_hypot missing (link-level) |
| [#1738](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1738) | __builtin_nexttoward missing (link-level) |
| [#1739](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1739) | __builtin_remainder missing (link-level) |
| [#1740](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1740) | __builtin_ldexp missing (link-level) |
| [#1741](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1741) | __builtin_exp2f missing (link-level) |
| [#1742](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1742) | __builtin_expm1f missing (link-level) |
| [#1743](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1743) | __builtin_log1pf missing (link-level) |
| [#1744](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1744) | __builtin_cbrtf missing (link-level) |
| [#1745](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1745) | __builtin_sinhf missing (link-level) |
| [#1746](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1746) | __builtin_tanf missing (link-level) |
| [#1747](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1747) | __builtin_asinf missing (link-level) |
| [#1748](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1748) | __builtin_atanf missing (link-level) |
| [#1749](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1749) | __builtin_tgammaf missing (link-level) |
| [#1750](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1750) | __builtin_erff missing (link-level) |
| [#1751](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1751) | __builtin_logbf missing (link-level) |
| [#1752](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1752) | __builtin_nearbyintf missing (link-level) |
| [#1753](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1753) | __builtin_rintf missing (link-level) |
| [#1754](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1754) | __builtin_log2f missing (link-level) |
| [#1755](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1755) | __builtin_log10f missing (link-level) |
| [#1756](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1756) | __builtin_log2l missing (link-level) |
| [#1757](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1757) | __builtin_strdup missing (link-level) |
| [#1758](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1758) | __builtin_roundeven missing (link-level) |
| [#1759](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1759) | __builtin_roundevenf missing (link-level) |
| [#1760](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1760) | __builtin_roundevenl missing (link-level) |
| [#1761](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1761) | __builtin_issignaling missing (link-level) |
| [#1762](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1762) | __builtin_clzg missing (link-level) |
| [#1763](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1763) | __builtin_ctzg missing (link-level) |
| [#1764](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1764) | __builtin_popcountg missing (link-level) |
| [#1765](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1765) | __builtin_modf missing (link-level) |
| [#1766](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1766) | __builtin_frexp missing (link-level) |
| [#1769](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1769) | __builtin_strpbrk missing (straggler) |
| [#1770](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1770) | __builtin_stpncpy missing (straggler) |
| [#1771](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1771) | __builtin_isnanf missing (b3/b4) |
| [#1772](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1772) | __builtin_isinff missing (b3/b4) |
| [#1773](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1773) | __builtin_isnanl missing (b3/b4) |
| [#1774](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1774) | __builtin_isinfl missing (b3/b4) |
| [#1775](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1775) | __builtin_cabs missing (b3/b4) |
| [#1776](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1776) | __builtin_cproj missing (b3/b4) |
| [#1777](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1777) | __builtin_carg missing (b3/b4) |
| [#1778](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1778) | __builtin_bcmp missing (b3/b4) |
| [#1779](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1779) | __builtin_bzero missing (b3/b4) |
| [#1780](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1780) | __builtin__exit missing (b3/b4) |
| [#1781](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1781) | __builtin_imaxabs missing (b3/b4) |
| [#1782](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1782) | __builtin_setjmp missing (b3/b4) |
| [#1783](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1783) | __builtin_longjmp missing (b3/b4) |
