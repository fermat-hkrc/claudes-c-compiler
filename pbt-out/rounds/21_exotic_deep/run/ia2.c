__attribute__((noinline)) int opaque(int x) { return x * 3; }
__attribute__((always_inline)) inline int fast(int x) { return opaque(x) + 1; }
int main(void) { printf("%d\n", fast(4) + fast(5)); return 0; }
