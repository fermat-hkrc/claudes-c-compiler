#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 83, 24, 221, 162, 103, 44, 241, 182, 123, 64, 5, 202, 143, 84, 25, 222 };
static double g1 = 'm';
static struct { int a; char b; double c; } g2 = {89889, 'x', 8171.727272727273};
static struct { unsigned x:1; unsigned y:31; } g3 = {0, 9142465630289317600};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
