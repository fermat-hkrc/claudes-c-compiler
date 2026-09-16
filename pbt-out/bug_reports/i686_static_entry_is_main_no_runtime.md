# Bug: ccc-i686 -static emits entry = main with no C runtime — every static i686 binary crashes at startup

**Law:** A statically linked executable's ELF entry point must be the runtime start function (`_start`), which sets up the stack/ABI environment, calls `main(argc, argv, envp)`, and terminates via the exit syscall with `main`'s return value. Emitting `main` itself as the entry is never valid: at process entry the stack top holds argc/argv, so `main`'s epilogue `ret` pops argc as a return address.

**Impact:** Every program compiled with `ccc -static` on the i686 backend segfaults immediately at startup, on any i686 environment (environment-independent — reproduced under qemu-i386 user-mode emulation). Exit status is 139 (SIGSEGV) instead of the program's return value. The x86-64 backend is unaffected (verified: `-static` links a proper `_start`); other backends untested.

**Function:** i686 standalone linker / static link pipeline (entry-point selection + crt0 selection)

**Detected by:** manual verification of issue #4 (fermat-hkrc/claudes-c-compiler#4) — needed a runnable static i686 binary to test cast codegen at runtime.

**Minimal input:**
```c
int main(void){return 7;}
```
```bash
ccc-i686 -static t7.c -o t7 && qemu-i386-static ./t7; echo $?   # → SIGSEGV, exit 139
```

**Expected:** program runs and exits 7. ELF entry points at a `_start` routine; the executable segment contains startup code plus `main`.

**Actual:** the binary crashes before reaching any program logic:

```console
$ readelf -h t7 | grep Entry
  Entry point address:               0x8049000
$ readelf -l t7 | grep "R E"
  LOAD           0x001000 0x08049000 0x08049000 0x00010 0x00010 R E 0x1000
```

The executable segment is exactly **0x10 = 16 bytes**, and disassembly of the entry is `main` itself with no runtime:

```
0:  55                   push   %ebp
1:  89 e5                mov    %esp,%ebp
3:  83 ec 08             sub    $0x8,%esp
6:  b8 07 00 00 00       mov    $0x7,%eax        ; return 7
b:  89 ec                mov    %ebp,%esp
d:  5d                   pop    %ebp
e:  c3                   ret                      ; pops argc (=1) → jumps to address 1 → SIGSEGV
```

No `_start`, no crt0, no exit syscall; the RW LOAD segment has FileSiz 0 (no data/bss support either).

**Root cause (probable):** the i686 static link path selects the `main` symbol as the ELF entry and links no startup object. The x86-64 static path links a real `_start` (verified: entry code is `endbr64; xor ebp,ebp; pop rsi; …`), so the defect is specific to the i686 static pipeline.

**Severity:** high (feature completely broken: any `-static` build on i686 is unusable)

**Regression test / verification harness:** a runtime probe can avoid both this bug and libc entirely by exiting via `int $0x80` inline asm (entry still lands on `main`, which then never returns) — that pattern is how issue #4's runtime verification was performed (`/tmp/i4_syscall.c` in the issue #4 verification record: `pbt-out/verified_bug/issue#4_Ptr_not_treated_as_unsigned_for_float_casts.md`).

**Related:** issue #4 (discovered during its verification). Environment note for reproducers: on hosts without 32-bit libc, *dynamic* ccc-i686 binaries also fail (missing loader deps) — use `qemu-i386-static`, which reproduces this bug in isolation.

## Found by

**Contract-Based Differential Validation (LLM verification session, 2026-09-15)** —
discovered as a side-finding while building a reproduction harness for issue #4.
(This bug itself was NOT found by pi-pbt; the command below is the campaign that found
issue #4, shown as origin context only:)

```bash
/usr/local/bin/pi-pbt build-run --provider zai-coding-cn --model glm-5.3 --lang en --build-cmd "cargo check --lib" \
  --scope src/backend/cast.rs \
  --func classify_cast_with_f128
```
