long add_asm(long a, long b) {
  long r;
  __asm__ volatile ("addq %1, %0" : "=r"(r) : "r"(b), "0"(a));
  return r;
}
long popcnt_asm(unsigned long long x) {
  unsigned long long r;
  __asm__ volatile ("popcntq %1, %0" : "=r"(r) : "r"(x));
  return r;
}
unsigned bswap_asm(unsigned x) {
  unsigned r;
  __asm__ ("movl %1, %0\n\tbswapl %0" : "=&r"(r) : "r"(x));
  return r;
}
int main(void) {
  unsigned long long v = 0xFF00FF00FF00FF00ULL;
  printf("%ld %lld %x %d\n", add_asm(3, 4), popcnt_asm(v), bswap_asm(0x12345678),
         ({ int x = 5; __asm__ volatile ("" : "+r"(x)); x; }));
  return 0;
}
