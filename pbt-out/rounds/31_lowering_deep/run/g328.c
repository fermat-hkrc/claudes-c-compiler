#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{276, 276, 1276}, {276, 1276}};
static unsigned char[16] g1 = { 202, 80, 214, 92, 226, 104, 238, 116, 250, 128, 6, 140, 18, 152, 30, 164 };
static unsigned char[16] g2 = { 237, 232, 227, 222, 217, 212, 207, 202, 197, 192, 187, 182, 177, 172, 167, 162 };
static double g3 = 'x';

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
