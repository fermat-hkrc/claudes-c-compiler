#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 194, 16, 94, 172, 250, 72, 150, 228, 50, 128, 206, 28, 106, 184, 6, 84 };
static int g1 = 395404;
static short[5] g2 = {10968, 68, 40968};
static int[4][3] g3 = {{413, 1413, 413}, {1413, 4413}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
