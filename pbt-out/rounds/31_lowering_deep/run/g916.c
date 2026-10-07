#include <stdio.h>
#include <string.h>

static double g0 = 'n';
static struct { int a; char b; double c; } g1 = {26017, 'z', 2365.181818181818};
static unsigned char[16] g2 = { 181, 40, 155, 14, 129, 244, 103, 218, 77, 192, 51, 166, 25, 140, 255, 114 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'u', 'v', 'w'}, 3350, {23350, 2914587740515403350}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
