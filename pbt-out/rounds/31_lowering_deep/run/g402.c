#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{371, 371, 2371}, {2371, 3371}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {7, 23, 5617527};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'u', 'v', 'w'}, 3248, {23248, -8408642388039448368}};
static union { long l; short s[4]; } g3 = {0xff28fb6ea51e0a3f};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
