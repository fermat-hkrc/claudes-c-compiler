# Round 35 — elf/driver/predefined_macros sweep

## Bugs Found
### #1092 — linker output drops .symtab/.strtab
- **Formal:** default link (no -s) must emit .symtab/.strtab with defined symbols (ld/lld/gcc behavior). FALSIFIED — only .dynsym (3 entries) + .shstrtab; nm → "no symbols"; no enabling flag.
- Bug report: bug_reports/linker_no_symtab.md
- Status: **filed #1092**.

## Properties / Probes (final status)
| Probe | Result | Status |
|-------|--------|--------|
| predefined macros (12 key: __STDC__, __STDC_VERSION__=201710L, __x86_64__, __linux__, __SIZEOF_*, __CHAR_BIT__, __BYTE_ORDER__) | all exact vs gcc | **passing** |
| driver modes (-E/-c/-S/.o-relink/a.out default) | all correct; .o is ELF relocatable, re-link byte-exact | **passing** |
| readelf -a structural health | 0 warnings/errors | **passing** |
| eh_frame parse (7 CIE/FDE) | 0 errors | **passing** |
| program headers (4 LOAD) | sane | **passing** |
| -ffunction-sections -Wl,--gc-sections | works, output identical | **passing** |
| .symtab presence in linked output | MISSING | **bug → #1092** |

## Design Caveats
- -S emits strings as .byte lists (odd style, functionally correct) — Doc evidence: /tmp/m1.s output, assembles and links back to identical behavior.
