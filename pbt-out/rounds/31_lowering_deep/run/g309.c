#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 245, 40, 91, 142, 193, 244, 39, 90, 141, 192, 243, 38, 89, 140, 191, 242 };
static int g1 = 963848;
static double g2 = 'v';
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 1816850698753976385};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
