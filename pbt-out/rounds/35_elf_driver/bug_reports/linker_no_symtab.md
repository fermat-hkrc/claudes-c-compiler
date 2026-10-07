# linker output has no .symtab/.strtab — nm reports "no symbols", gdb/perf symbolization impossible, no flag to enable
> **Filed as [issue #1092](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1092)**

# Bug: linked executables drop the static symbol table entirely

**Formal:** A default link (no -s/--strip-all) must emit .symtab/.strtab carrying the input objects' defined symbols (main, globals, functions) — matching ld/lld/gcc behavior. FALSIFIED: output contains only .dynsym (3 entries: null, printf, __libc_start_main) + .shstrtab; every defined symbol is unrepresentable.

**Impact:** Toolchain interop failure on every ccc-linked binary:
```
$ nm elf_check
nm: elf_check: no symbols

$ readelf -SW elf_check | grep -E 'symtab|strtab'
  [25] .shstrtab ...        <- only section-name table; no .symtab/.strtab

$ readelf -sW elf_check
Symbol table '.dynsym' contains 3 entries: null, printf@GLIBC, __libc_start_main@GLIBC
```
- gdb: no function names, breakpoints by name impossible
- perf/addr2line/backtrace_symbolize: unresolved addresses
- nm/objdump -t: empty
- No CLI flag exists to enable it (checked --help: no strip/symbol/debug options) — the user has NO path to a symbolized binary. Input .o files DO have correct .symtab (9 entries, nm shows main/helper/shared_val), so the loss happens in the link step.

**Root cause:** the executable writer never emits SHT_SYMTAB/SHT_STRTAB. linker_common's gc_sections.rs/merge.rs/parse_object.rs handle SHT_SYMTAB of *input* objects (symbol resolution works — multi-TU linking is byte-exact vs gcc in 200-case PBT), but the output side (executable assembly path) skips static symbol emission entirely.

**Separate from:** dedup "symtab|nm no symbols|debug symbol missing|gdb symbol" = 0 hits across ~1090 issues.

**Detected by:** round-35 elf structural health sweep (readelf -a/-s/-SW differential vs gcc binary; gcc output has 2 symbol tables with all 4 defined symbols visible).

**Minimal input:**
```bash
echo 'int main(){return 0;}' > t.c
ccc t.c -o t && nm t    # "no symbols" — expected: 0000... T main
```

**Fix sketch:** after section layout, build .symtab from all defined input symbols (resolved addresses, LOCAL/GLOBAL binding, FUNC/OBJECT types, sizes) + .strtab, mark non-ALLOC so they don't load at runtime (matches ld).

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '#include <stdio.h>\nextern int shared_val;\nextern int helper(int);\nint main(){printf("%d %d\\n", shared_val, helper(5)); return 0;}' > /tmp/m1.c
printf 'int shared_val = 42;\nint helper(int x){return x*2;}' > /tmp/m2.c
./target/debug/ccc /tmp/m1.c /tmp/m2.c -o /tmp/elf_check
readelf -sW /tmp/elf_check   # only .dynsym
nm /tmp/elf_check            # no symbols
```
