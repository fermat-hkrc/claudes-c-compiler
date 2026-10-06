void bump(int *p) { *p += 10; }
int main(void) {
  int x = 5;
  int *px = &x;        // 地址逃逸 → 不提升
  bump(px);
  bump(&x);            // 直接取地址
  printf("%d %d\n", x, *px);
  return 0;
}
