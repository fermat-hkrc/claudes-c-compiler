#include <stdio.h>
#include <string.h>

static long long g0 = 3579712402u;
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {7, 15, 5583887};
static float g2 = 48734;
static unsigned char[16] g3 = { 120, 192, 8, 80, 152, 224, 40, 112, 184, 0, 72, 144, 216, 32, 104, 176 };

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
