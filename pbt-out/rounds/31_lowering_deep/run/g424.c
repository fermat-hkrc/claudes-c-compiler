#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xcf07bfc6ea40e47d};
static long long g1 = 3775965182u;
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'', '', ''}, 26339, {6339, 2329665359317926339}};
static short[5] g3 = {5382, 82, 35382};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
