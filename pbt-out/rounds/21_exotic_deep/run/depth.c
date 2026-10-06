#include <stdio.h>
#include <stdlib.h>
long sumto(long n, long acc) { return n == 0 ? acc : sumto(n - 1, acc + n); }
int main(int argc, char **argv) { printf("%ld\n", sumto(atol(argv[1]), 0)); return 0; }
