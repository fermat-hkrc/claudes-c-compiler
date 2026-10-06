long sumto(long n, long acc) { return n == 0 ? acc : sumto(n - 1, acc + n); }
int main(void) { printf("%ld\n", sumto(1000000, 0)); return 0; }
