#include <stdio.h>
long sum_with_invariant(int n, int k) {
    long invariant = k * 1000 + 42;
    long sum = 0;
    for (int i = 0; i < n; i++) {
        sum += invariant + (i % 7);      // invariant hoisting + modulo
        invariant = invariant;           // self-assign (GVN should eliminate)
    }
    return sum;
}
int main(void) { printf("%ld %ld\n", sum_with_invariant(10, 5), sum_with_invariant(0, 3)); return 0; }
