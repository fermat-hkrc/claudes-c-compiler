#!/bin/bash
CCC=/home/shuhao/fermat-users/leo/github/claudes-c-compiler/target/release/ccc
while IFS= read -r s; do
  printf '%s\n' ".text" "$s" "target:" "	ret" > mb.s
  $CCC -c -o mb_c.o mb.s 2>/dev/null; ccc=$?
  as -o mb_g.o mb.s 2>/dev/null; gas=$?
  llvm-mc -triple=x86_64 -filetype=obj -o mb_m.o mb.s 2>/dev/null; mc=$?
  mark=""
  [ $ccc -eq 0 ] && [ $gas -ne 0 ] && [ $mc -ne 0 ] && mark=" <<BUG"
  [ $ccc -ne 0 ] && [ $gas -eq 0 ] && [ $mc -eq 0 ] && mark=" <<GAP"
  printf '%-38s ccc=%d as=%d mc=%d%s\n' "$s" $ccc $gas $mc "$mark"
  rm -f mb_c.o mb_g.o mb_m.o
done
