#include <setjmp.h>
jmp_buf buf;
int depth(int n) {
  if (n == 0) longjmp(buf, 42);
  return depth(n - 1) + 1;
}
int main(void) {
  int v = setjmp(buf);
  if (v == 0) { depth(10); printf("unreachable\n"); return 1; }
  printf("caught %d\n", v);
  return 0;
}
