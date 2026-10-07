#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{320, 320, 320}, {2320, 2320}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'o', 'p', 'q'}, 61694, {11694, 6087221133200561694}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {2, 18, 6904978};
static unsigned char[16] g3 = { 60, 224, 132, 40, 204, 112, 20, 184, 92, 0, 164, 72, 236, 144, 52, 216 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
