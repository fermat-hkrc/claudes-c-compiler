#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {74073, 'r', 6733.909090909091};
static struct { int a; char b; double c; } g1 = {86665, '|', 7878.636363636364};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'p', 'q', 'r'}, 98677, {28677, -1558142739126652939}};
static short[5] g3 = {22971, 71, 42971};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
