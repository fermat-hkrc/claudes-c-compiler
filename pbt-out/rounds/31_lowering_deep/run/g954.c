#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 215, 56, 153, 250, 91, 188, 29, 126, 223, 64, 161, 2, 99, 196, 37, 134 };
static char[8] g1 = {'l', 'L', '', 27};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {0, 24, 3878808};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'~', '', ''}, 35823, {25823, 8099604669464035823}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
