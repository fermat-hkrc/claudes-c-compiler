#include <stdio.h>
#include <string.h>

static struct { unsigned a:3; unsigned b:5; int c:24; } g0 = {7, 7, -667545};
static int g1 = 403069;
static int g2 = 807333;
static char[8] g3 = {'z', 'Z', '', 117};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
