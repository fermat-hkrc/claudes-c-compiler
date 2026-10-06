int main(void) {
  double oz = -0.0, a = 0.0; double neg = -a;
  printf("%f %f %f %d %d %d\n", oz, neg, 1.0/oz, oz < 0, neg == 0, 1.0/neg < 0);
  return 0;
}
