#include <stdio.h>
static long collatz(long n, int steps) {
    if (n == 1) return steps;
    if (n % 2 == 0) return collatz(n / 2, steps + 1);
    return collatz(3 * n + 1, steps + 1);
}
static int classify(int x) {
    static const int jump_val[] = { 10, 20, 30, 40, 50 };
    if (x >= 0 && x < 5) return jump_val[x];
    return -1;
}
int main(void) {
    for (int i = 1; i <= 10; i++)
        printf("%ld ", collatz(i, 0));
    printf("\n");
    for (int i = -1; i <= 6; i++)
        printf("%d ", classify(i));
    printf("\n");
    return 0;
}
