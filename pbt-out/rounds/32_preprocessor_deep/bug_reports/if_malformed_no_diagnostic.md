# #if accepts malformed expressions with no diagnostic — 8/13 silently evaluate into a branch (e.g. '#if 1 +' takes TRUE)
> **Filed as [issue #1088](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1088)**

# Bug: #if expression parser accepts invalid syntax, zero diagnostics, wrong branch

**Formal:** For every #if expression E that is not a valid integer constant expression: the preprocessor must emit a diagnostic (C11 6.10.1p4 constraint violation) and must not silently select a branch. FALSIFIED: 12/13 malformed expressions produce zero diagnostics; 8 silently select the TRUE branch.

**Impact:** A typo in a real header silently compiles the wrong variant instead of erroring:
```
expression     gcc              ccc
#if ^2         error            FALSE branch (no diagnostic)
#if 1 +        error: expected  TRUE branch   <- truncated expr silently true
#if (1         error            TRUE branch   <- unbalanced paren
#if !          error            TRUE branch
#if 1 ++ 2     error            TRUE branch
#if ~          error            TRUE branch
#if #1         error            TRUE branch
#if 1 2        error            TRUE branch   <- adjacent literals
#if x = 5      error            FALSE branch  <- assignment accepted
#if **3        error            FALSE branch
#if ??         error            FALSE branch
```
(All gcc errors are "#if with no expression"/"token is not valid in preprocessor expressions" family; ccc emits nothing and picks a branch.)

**Root cause:** expr_eval.rs evaluation path — after `replace_remaining_idents_with_zero` and `resolve_defined_in_expr`, the token-to-value parse treats unknown/truncated token sequences as 0/operand-lookup failure without a hard error; partial stacks resolve instead of failing.

**Separate from:** no existing issue mentions `#if` or `expr_eval` (dedup: 0 hits across ~1085 titles+bodies).

**Detected by:** round-32 preprocessor deep PBT — `p_if_corners_vs_gcc` skip analysis (gcc=None, ccc=Some) + manual sweep of 13 malformed forms via `ccc -E` vs `gcc -E -P`.

**Minimal input:**
```c
#if 1 +
int wrong_branch_selected;
#endif
```

**Fix sketch:** the #if expression parser needs a strict mode: any unconsumed token, unbalanced paren, dangling operator, or non-expression token must return Err → emit diagnostic (matching gcc behavior of failing the conditional rather than guessing).

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
for e in '^2' '1 +' '(1' '!' '1 ++ 2' '~' '#1' '1 2'; do
  printf "#if %s\nT\n#else\nF\n#endif\n" "$e" > /tmp/t.c
  gcc -E -P /tmp/t.c 2>&1 | head -1   # error
  ./target/debug/ccc -E /tmp/t.c 2>&1 | head -1  # silently T
done
```
