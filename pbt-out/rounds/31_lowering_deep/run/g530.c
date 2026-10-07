#include <stdio.h>
#include <string.h>

static struct { char x[3]; int y; struct { short p; long q; } z; } g0 = {{'', '', ''}, 53667, {3667, -4232771128236797949}};
static unsigned char[16] g1 = { 87, 56, 25, 250, 219, 188, 157, 126, 95, 64, 33, 2, 227, 196, 165, 134 };
static long long g2 = 253472124u;
static unsigned char[16] g3 = { 133, 168, 203, 238, 17, 52, 87, 122, 157, 192, 227, 6, 41, 76, 111, 146 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
