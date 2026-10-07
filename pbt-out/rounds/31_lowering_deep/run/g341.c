#include <stdio.h>
#include <string.h>

static float g0 = -37632;
static unsigned char[16] g1 = { 31, 120, 209, 42, 131, 220, 53, 142, 231, 64, 153, 242, 75, 164, 253, 86 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'~', '', ''}, 25341, {15341, -1302573828272226275}};
static union { int i; char c[8]; double d; } g3 = {0x56826ce8};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
