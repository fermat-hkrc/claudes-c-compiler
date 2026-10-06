int main(void) {
  volatile int v = 0;
  for (int i = 0; i < 5; i++) v = v + i;
  int alias[1]; int *p = alias;
  printf("%d\n", v);
  return 0;
}
