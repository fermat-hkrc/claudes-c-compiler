int pick(int c, int a, int b) {
  int r;
  if (c) r = a * 2; else r = b + 3;
  return r + (c ? r - 1 : r + 1);
}
int main(void) { printf("%d %d\n", pick(1, 10, 20), pick(0, 10, 20)); return 0; }
