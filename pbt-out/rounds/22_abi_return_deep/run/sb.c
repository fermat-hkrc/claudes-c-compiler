#include <math.h>
double gv_neg = -0.0;
int main(void) {
  double lit = -0.0;          // 常量初始化的局部
  double computed = 0.0 * -1.0; // 运行时算出
  float f32 = -0.0f;
  printf("%d %d %d %d %d\n",
    signbit(gv_neg), signbit(lit), signbit(computed), signbit((double)f32),
    signbit(3.5) + 2*signbit(-3.5));
  printf("%d %d\n", __builtin_signbit(-0.0), __builtin_signbit(0.0));
  return 0;
}
