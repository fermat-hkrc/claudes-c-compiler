#include <math.h>
int main(void) {
  double z = 0.0, oz = -0.0;
  double inf = 1.0 / z, nan = z / z;
  printf("%d %d %d %d %d %d %d\n",
    nan == nan, nan != nan, nan < 1.0, isnan(nan),
    signbit(oz), inf > 1e308, (double)(float)nan != nan);
  return 0;
}
