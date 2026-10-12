# Issue #1787 — C95 digraphs (<% %> <: :> %:) entirely unrecognized

**Law:** C95 alternative token spellings
**Witness:** int main(void) <% ... %> -> ccc parse error; gcc prints digraph-ok
**Reproduce:**
```bash
ccc dg2.c -o x   # parse error at <%
gcc dg2.c -o y && ./y   # digraph-ok
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1787
