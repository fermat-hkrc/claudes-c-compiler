#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{566, 566, 566}, {2566, 2566}};
static union { long l; short s[4]; } g1 = {0xf95f5b696e8e04f4};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'|', '}', '~'}, 7801, {7801, -7979188250351443815}};
static int g3 = -557846;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
