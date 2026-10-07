#include <stdio.h>
#include <string.h>

static int g0 = 92320;
static long long g1 = 3585558338u;
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'v', 'w', 'x'}, 62423, {12423, -9127670026643289193}};
static short[5] g3 = {20213, 13, 20213};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
