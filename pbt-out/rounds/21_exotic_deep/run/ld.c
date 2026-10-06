int main(void) {
  long double a = 1.0L / 3.0L, b = a * 3.0L;
  long double c = sqrtl(2.0L) * sqrtl(2.0L);
  printf("%d %d %.10Lg\n", b == 1.0L, (int)(c == 2.0L), a);
  return 0;
}
