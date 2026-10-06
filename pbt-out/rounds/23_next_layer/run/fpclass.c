#include <math.h>
double gz = 0.0, gn = -0.0, gi = INFINITY, gni = -INFINITY, gn1 = -1.5, g05 = 0.5;
int main(void) {
  // 常量直接内联（ccc 会折叠） vs 全局（运行时路径，应正确）
  printf("const: %d%d%d%d%d%d\n",
    signbit(-0.0), isnan(0.0/0.0), isinf(1.0/0.0), signbit(-INFINITY),
    isnan(-0.0)==0, !!isnormal(0.5));
  printf("glob : %d%d%d%d%d%d\n",
    signbit(gn), isnan(gz/gz), isinf(gi), signbit(gni),
    isnan(gn)==0, !!isnormal(g05));
  return 0;
}
