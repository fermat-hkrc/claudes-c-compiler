#include <stdio.h>
#include <string.h>

static char[8] g0 = {'l', 'L', '', 65};
static union { int i; char c[8]; double d; } g1 = {0xd55b836d};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'m', 'n', 'o'}, 11750, {11750, -8018463497824239866}};
static int g3 = -735699;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
