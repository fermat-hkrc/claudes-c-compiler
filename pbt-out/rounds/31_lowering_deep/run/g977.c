#include <stdio.h>
#include <string.h>

static short[5] g0 = {28860, 60, 28860};
static unsigned char[16] g1 = { 108, 96, 84, 72, 60, 48, 36, 24, 12, 0, 244, 232, 220, 208, 196, 184 };
static char[8] g2 = {'z', 'Z', '', 161};
static unsigned char[16] g3 = { 221, 104, 243, 126, 9, 148, 31, 170, 53, 192, 75, 214, 97, 236, 119, 2 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
