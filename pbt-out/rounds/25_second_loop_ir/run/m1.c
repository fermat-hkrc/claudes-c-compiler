struct B7 { char b[7]; }; struct B9 { char b[9]; }; struct B16 { char b[16]; };
int main(void) {
  struct B7 a = {{1}}; struct B9 b = {{2}}; struct B16 c = {{3}};
  a.b[6] = 7; b.b[8] = 9; c.b[15] = 16;
  printf("%d %d %d\n", a.b[6], b.b[8], c.b[15]);
  return 0;
}
