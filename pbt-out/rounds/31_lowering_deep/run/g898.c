#include <stdio.h>
#include <string.h>

static short[5] g0 = {1755, 55, 61755};
static int g1 = 469403;
static struct { unsigned x:1; unsigned y:31; } g2 = {0, 5103389310915674016};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'', '', ''}, 94671, {4671, -7057078208627656945}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
