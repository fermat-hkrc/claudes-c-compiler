#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 156, 224, 36, 104, 172, 240, 52, 120, 188, 0, 68, 136, 204, 16, 84, 152 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {4, 28, 4046844};
static int g2 = -314082;
static long long g3 = 756925077u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
