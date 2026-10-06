# Bug: i686 target strips the arch-neutral gcc-internal include path — `#include <stdio.h>` fails out of the box on multilib hosts
**Law:** switching targets must keep the compiler able to find the freestanding headers (stddef.h, stdarg.h). On a standard x86-64 multilib host their only source is `/usr/lib/gcc/x86_64-linux-gnu/<v>/include` — arch-NEUTRAL files that the i686 target can and must use.
**Impact:** `ccc -m32` / `ccc-i686` cannot compile ANY program that includes a glibc header (stdio.h pulls stddef.h/stdarg.h): "stddef.h: No such file or directory". The i686 target is unusable out of the box on the standard setup (gcc + gcc-multilib). Round 12's E2E property needed `-I /usr/lib/gcc/x86_64-linux-gnu/10/include` to run at all.
**Function:** PredefinedMacros::set_target, "i686" | "i386" arm
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/preprocessor/predefined_macros.rs:187-190 (`system_include_paths.retain(|p| !p.contains("x86_64"))`) followed by the i686 path list (:191-202) whose cross dirs (`/usr/lib/gcc-cross/i686-linux-gnu/...`, `/usr/i686-linux-gnu/include`) do not exist on multilib hosts.
**Detected by:** round-12 E2E harness bring-up (compile failure, not a property mismatch)
**Minimal input:** `echo '#include <stdio.h>
int main(void){return 0;}' > t.c && ccc -m32 -o t t.c` → /usr/include/stdio.h:33:10: error: stddef.h: No such file or directory
**Expected:** compiles (like the x86-64 target does, finding the same file in /usr/lib/gcc/x86_64-linux-gnu/<v>/include).
**Actual:** 2 preprocessor errors; the retain() removed the only dir containing stddef.h/stdarg.h.
**Severity:** medium (i686 target unusable out of the box; worked around by -I)
**Contract evidence:** inferred (freestanding headers are target-agnostic — gcc's own stddef.h is shared across -m32/-m64 on multilib hosts; the ARM/RISC-V arms legitimately swap cross dirs, but the i686 case runs ON the host gcc installation whose internal include dir is arch-neutral)
**Fix:** in the i686 arm, retain the gcc-INTERNAL dirs (only strip the multiarch `/usr/include/x86_64-linux-gnu`), or append the existing `/usr/lib/gcc/x86_64-linux-gnu/<v>/include` dirs to the i686 list.
**Regression test:** (deterministic check) `ccc -m32` compiles `#include <stddef.h>` without -I — recorded in the round-12 harness comment; a unit regression needs the fixed path list (would assert set_target("i686") keeps a path that resolves stddef.h on the build host).

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '#include <stdio.h>\nint main(void){return 0;}\n' > /tmp/t32.c
PATH="$HOME/.cargo/bin:$PATH" cargo build --release
./target/release/ccc -m32 -o /tmp/t32 /tmp/t32.c
```
**Raw output:**
```
/usr/include/stdio.h:33:10: error: stddef.h: No such file or directory
/usr/include/stdio.h:36:10: error: stdarg.h: No such file or directory
ccc: error: 2 preprocessor error(s) in /tmp/t32.c
```
