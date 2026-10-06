#include <setjmp.h>
static jmp_buf b;
static volatile int keep = 0;   // volatile: must survive longjmp
static int lost = 0;            // non-volatile: indeterminate after longjmp (gcc keeps at O0)
int main(void) {
  int v = setjmp(b);
  if (v == 0) { keep = 7; lost = 7; longjmp(b, 1); }
  printf("%d %d\n", keep, lost);
  return 0;
}
