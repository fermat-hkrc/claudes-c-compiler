#include <stdio.h>
#include <string.h>

static int g0 = -394998;
static char[8] g1 = {'g', 'G', '~', 196};
static long long g2 = 2691912276u;
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'n', 'o', 'p'}, 88385, {28385, 6165876838538788385}};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
