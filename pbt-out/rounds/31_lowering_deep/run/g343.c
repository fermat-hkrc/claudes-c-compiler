#include <stdio.h>
#include <string.h>

static short[5] g0 = {11250, 50, 31250};
static unsigned char[16] g1 = { 93, 104, 115, 126, 137, 148, 159, 170, 181, 192, 203, 214, 225, 236, 247, 2 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'m', 'n', 'o'}, 14834, {4834, -5780094444709636782}};
static long long g3 = 1993408027u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
