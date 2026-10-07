#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 124, 224, 68, 168, 12, 112, 212, 56, 156, 0, 100, 200, 44, 144, 244, 88 };
static int[4][3] g1 = {{273, 1273, 2273}, {1273, 273}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'t', 'u', 'v'}, 9857, {19857, 3518671797310909857}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g3 = {7, 31, 3709151};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
