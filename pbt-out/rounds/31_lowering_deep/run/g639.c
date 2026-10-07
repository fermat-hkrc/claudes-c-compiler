#include <stdio.h>
#include <string.h>

static int g0 = -243812;
static long long g1 = 3641568806u;
static short[5] g2 = {27118, 18, 7118};
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 3646257387442096349};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
