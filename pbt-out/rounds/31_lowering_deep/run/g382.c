#include <stdio.h>
#include <string.h>

static struct { unsigned x:1; unsigned y:31; } g0 = {1, 7281918629729431175};
static char[8] g1 = {'a', 'A', 'x', 186};
static unsigned char[16] g2 = { 210, 144, 78, 12, 202, 136, 70, 4, 194, 128, 62, 252, 186, 120, 54, 244 };
static short[5] g3 = {3510, 10, 43510};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
