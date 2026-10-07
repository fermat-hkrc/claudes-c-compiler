#include <stdio.h>
#include <string.h>

static int g0 = -890426;
static long long g1 = 2384819678u;
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'n', 'o', 'p'}, 10065, {10065, -4838117665653041551}};
static unsigned char[16] g3 = { 105, 200, 39, 134, 229, 68, 163, 2, 97, 192, 31, 126, 221, 60, 155, 250 };

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
