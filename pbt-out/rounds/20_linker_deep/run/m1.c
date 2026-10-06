extern int shared_val;
extern long mul2(long);
int get7(void) { return 7; }
int main(void) {
  printf("%d %ld %d\n", shared_val, mul2(21), get7());
  return 0;
}
