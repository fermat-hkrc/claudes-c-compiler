int main(void) {
  int i = 0, acc = 1;
  while (i < 10) { acc = acc + i; i = i + 1; }
  printf("%d %d\n", i, acc);
  return 0;
}
