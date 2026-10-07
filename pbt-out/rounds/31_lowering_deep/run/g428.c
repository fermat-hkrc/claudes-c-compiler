#include <stdio.h>
#include <string.h>

static long long g0 = 2231919949u;
static char[8] g1 = {'c', 'C', 'z', 42};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'v', 'w', 'x'}, 6033, {6033, -7084483861345245583}};
static char[8] g3 = {'k', 'K', '', 128};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
