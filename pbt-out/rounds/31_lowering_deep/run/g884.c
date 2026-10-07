#include <stdio.h>
#include <string.h>

static long long g0 = 3980141845u;
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'', '', ''}, 82025, {22025, 503905640408482025}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'o', 'p', 'q'}, 86638, {6638, -5079991610645564978}};
static short[5] g3 = {4840, 40, 4840};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
