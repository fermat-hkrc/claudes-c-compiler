#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 7, 184, 105, 26, 203, 124, 45, 222, 143, 64, 241, 162, 83, 4, 181, 102 };
static struct { int a; char b; double c; } g1 = {43621, '', 3965.5454545454545};
static struct { int a; char b; double c; } g2 = {60600, '', 5509.090909090909};
static short[5] g3 = {1371, 71, 21371};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
