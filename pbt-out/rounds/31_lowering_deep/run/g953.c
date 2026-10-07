#include <stdio.h>
#include <string.h>

static long long g0 = 1889500179u;
static int g1 = 271554;
static unsigned char[16] g2 = { 3, 152, 45, 194, 87, 236, 129, 22, 171, 64, 213, 106, 255, 148, 41, 190 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g3 = {1, 17, -757007};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
