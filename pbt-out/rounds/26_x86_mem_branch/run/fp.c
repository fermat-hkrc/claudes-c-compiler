#include <stdio.h>
int main(void) {
  double inf = 1.0/0.0, nan = 0.0/0.0, zero = 0.0, nz = -0.0;
  volatile double x = inf, y = nan;
  printf("%d %d %d %d %d %d %d %d\n",
    x < 1e300, y < 1.0, y > 1.0, y == y, y != y,
    zero == nz, 1/zero > 0, -1/zero < 0);
  return 0;
}
