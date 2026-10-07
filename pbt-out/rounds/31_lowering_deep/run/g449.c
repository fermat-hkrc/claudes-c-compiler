#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 25, 72, 119, 166, 213, 4, 51, 98, 145, 192, 239, 30, 77, 124, 171, 218 };
static char[8] g1 = {'p', 'P', '', 13};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {7, 23, 7025687};
static long long g3 = 1842996032u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
