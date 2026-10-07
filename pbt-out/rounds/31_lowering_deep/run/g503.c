#include <stdio.h>
#include <string.h>

static struct { char x[3]; int y; struct { short p; long q; } z; } g0 = {{'', '', ''}, 23422, {3422, 8741727777090123422}};
static int[4][3] g1 = {{185, 1185, 1185}, {1185, 1185}};
static short[5] g2 = {26267, 67, 26267};
static struct { int a; char b; double c; } g3 = {87604, '{', 7964};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
