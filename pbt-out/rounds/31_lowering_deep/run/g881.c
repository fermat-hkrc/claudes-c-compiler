#include <stdio.h>
#include <string.h>

static struct { unsigned a:3; unsigned b:5; int c:24; } g0 = {5, 21, -2412395};
static int g1 = -34153;
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'x', 'y', 'z'}, 95769, {25769, 7941364960634395769}};
static int[4][3] g3 = {{539, 539, 1539}, {539, 539}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
