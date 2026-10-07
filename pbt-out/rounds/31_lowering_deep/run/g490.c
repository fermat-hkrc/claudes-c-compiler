#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {51244, '', 4658.545454545455};
static unsigned char[16] g1 = { 125, 104, 83, 62, 41, 20, 255, 234, 213, 192, 171, 150, 129, 108, 87, 66 };
static struct { int a; char b; double c; } g2 = {57225, '', 5202.272727272727};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'', '', ''}, 11306, {1306, 5410434375128911306}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
