#include <stdio.h>
#include <string.h>

static long long g0 = 2581481269u;
static long long g1 = 502091247u;
static int[4][3] g2 = {{178, 1178, 1178}, {3178, 178}};
static short[5] g3 = {25164, 64, 55164};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
