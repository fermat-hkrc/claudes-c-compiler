# Issue #1695 — No PT_GNU_RELRO segment — GOT/.dynamic permanently writable

**Law:** GNU ld emits PT_GNU_RELRO; loader mprotects it read-only
**Witness:** readelf -lW on any ccc binary: zero GNU_RELRO lines; gcc same-source binary has the segment
**Reproduce:**
```bash
printf 'int main(){return 0;}' > x.c && ccc x.c -o x && readelf -lW x | grep -c GNU_RELRO   # 0
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1695
