#include <stdio.h>
long stress(int a0, int a1, int a2, int a3, int a4, int a5,
            int b0, int b1, int b2, int b3, int b4, int b5) {
    // 12 params already in registers + stack
    long c0 = a0 + b0, c1 = a1 + b1, c2 = a2 + b2, c3 = a3 + b3;
    long c4 = a4 + b4, c5 = a5 + b5, c6 = a0 * b1, c7 = a1 * b2;
    long c8 = a2 * b3, c9 = a3 * b4, c10 = a4 * b5, c11 = a5 * b0;
    // Force a call in the middle (clobbers caller-saved)
    printf(".");
    // Now use all values (all must be live across the call)
    return c0+c1+c2+c3+c4+c5+c6+c7+c8+c9+c10+c11;
}
int main(void) { printf("%ld\n", stress(1,2,3,4,5,6, 7,8,9,10,11,12)); return 0; }
