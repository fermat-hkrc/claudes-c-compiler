#include <stdio.h>
#include <string.h>

static int g0 = 197046;
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {3, 3, 904899};
static long long g2 = 1162510979u;
static char[8] g3 = {'x', 'X', '', 225};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
