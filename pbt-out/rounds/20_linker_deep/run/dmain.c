extern long lib_mul(long, long);
extern long lib_state;
int main(void) {
  printf("%ld %ld\n", lib_mul(6, 7), lib_state);
  return 0;
}
