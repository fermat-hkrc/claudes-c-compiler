#include <stdarg.h>
long s(int n, ...) { va_list ap; va_start(ap, n); long t = 0; for (int i = 0; i < n; i++) t += va_arg(ap, int); va_end(ap); return t; }
int main(void) { printf("%ld %ld\n", s(3, 1, 2, 3), s(5, 10, 20, 30, 40, 50)); return 0; }
