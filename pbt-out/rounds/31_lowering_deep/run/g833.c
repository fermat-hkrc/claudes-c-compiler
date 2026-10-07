#include <stdio.h>
#include <string.h>

static char[8] g0 = {'x', 'X', '', 173};
static struct { int a; char b; double c; } g1 = {49692, '}', 4517.454545454545};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'', '', ''}, 45129, {5129, 8817184348908845129}};
static unsigned char[16] g3 = { 204, 96, 244, 136, 28, 176, 68, 216, 108, 0, 148, 40, 188, 80, 228, 120 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
