#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {96104, '', 8736.727272727272};
static char[8] g1 = {'n', 'N', '', 45};
static unsigned char[16] g2 = { 125, 104, 83, 62, 41, 20, 255, 234, 213, 192, 171, 150, 129, 108, 87, 66 };
static unsigned char[16] g3 = { 169, 200, 231, 6, 37, 68, 99, 130, 161, 192, 223, 254, 29, 60, 91, 122 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
