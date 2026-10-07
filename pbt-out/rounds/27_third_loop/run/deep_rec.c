#include <stdio.h>
volatile long depth = 0;
void recurse(long n) { depth = n; if (n > 0) recurse(n - 1); }
int main(void) {
    recurse(50000);
    printf("%ld\n", depth);
    return 0;
}
