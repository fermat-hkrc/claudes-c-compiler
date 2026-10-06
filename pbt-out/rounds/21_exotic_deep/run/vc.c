#include <stdarg.h>
long combine(int n, ...) {
  va_list ap, ap2;
  va_start(ap, n);
  va_copy(ap2, ap);
  long t = 0;
  for (int i = 0; i < n; i++) t += va_arg(ap, int);
  va_arg(ap2, int); va_arg(ap2, int);
  long u = (n > 1) ? va_arg(ap2, double), 0L : 0L;
  va_end(ap); va_end(ap2);
  return t;
}
int main(void) { printf("%ld\n", combine(4, 10, 20, 30, 40)); return 0; }
