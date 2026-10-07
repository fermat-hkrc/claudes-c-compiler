#include <stdio.h>
#include <string.h>

static long long g0 = 2565179859u;
static unsigned char[16] g1 = { 93, 104, 115, 126, 137, 148, 159, 170, 181, 192, 203, 214, 225, 236, 247, 2 };
static int[4][3] g2 = {{982, 1982, 2982}, {3982, 4982}};
static struct { int a; char b; double c; } g3 = {57848, 'q', 5258.909090909091};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
