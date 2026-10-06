int f(int x) {
  switch (x) {
    case 0: return 100; case 1000000: return 200; case 2000000: return 300;
    case 7: return 7; case 8: return 8; case 9: return 9;
    default: return -1;
  }
}
int main(void) { printf("%d %d %d %d %d %d %d\n", f(0), f(1000000), f(2000000), f(7), f(8), f(9), f(55)); return 0; }
