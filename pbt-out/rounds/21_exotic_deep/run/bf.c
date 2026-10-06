struct SB { int a : 3; unsigned b : 3; signed c : 2; };
int main(void) {
  struct SB x = { -4, 7, -2 };
  x.a--; x.b--; x.c--;
  printf("%d %d %d %d\n", x.a, x.b, x.c, (int)sizeof(struct SB));
  return 0;
}
