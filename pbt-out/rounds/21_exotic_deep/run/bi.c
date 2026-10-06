int main(void) {
  unsigned x = 0x12345678u;
  long long v = -1;
  printf("%u %d %d %d %d %d %d\n", __builtin_bswap32(x),
    __builtin_clz(1u), __builtin_ctz(64u), __builtin_popcount(255u),
    __builtin_clzll(v ^ (v + 1)), __builtin_constant_p(42), __builtin_expect(1, 1));
  return 0;
}
