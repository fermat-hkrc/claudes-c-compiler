#include <complex.h>
int main(void) {
  double complex a = 3.0 + 4.0*I, b = 1.0 - 2.0*I;
  double complex s = a + b, p = a * b;
  printf("%.1f%+.1fi %.1f%+.1fi %.1f\n", creal(s), cimag(s), creal(p), cimag(p), cabs(a));
  float complex f = 1.0f + 2.0f*I;
  printf("%.1f\n", (double)cabsf(f * ~f));
  return 0;
}
