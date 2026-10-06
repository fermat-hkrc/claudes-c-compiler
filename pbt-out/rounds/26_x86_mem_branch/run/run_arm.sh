#!/bin/bash
CCC_ARM=/home/shuhao/fermat-users/leo/github/claudes-c-compiler/target/release/ccc-arm
while IFS= read -r s; do
  printf '%s\n' ".text" "$s" > ab.s
  $CCC_ARM -mlittle-endian -c -o ab_c.o ab.s 2>/dev/null; ccc=$?
  aarch64-linux-gnu-as -o ab_g.o ab.s 2>/dev/null; gas=$?
  llvm-mc -triple=aarch64 -filetype=obj -o ab_m.o ab.s 2>/dev/null; mc=$?
  mark=""
  [ $ccc -eq 0 ] && [ $gas -ne 0 ] && [ $mc -ne 0 ] && mark=" <<BUG"
  [ $ccc -ne 0 ] && [ $gas -eq 0 ] && [ $mc -eq 0 ] && mark=" <<GAP"
  printf '%-34s ccc=%d as=%d mc=%d%s\n' "$s" $ccc $gas $mc "$mark"
  rm -f ab_c.o ab_g.o ab_m.o
done
