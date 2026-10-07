#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 214, 176, 138, 100, 62, 24, 242, 204, 166, 128, 90, 52, 14, 232, 194, 156 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'v', 'w', 'x'}, 15853, {15853, -2338565033857535763}};
static double g2 = 'q';
static double g3 = 'c';

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
