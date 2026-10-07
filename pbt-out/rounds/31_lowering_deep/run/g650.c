#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {84769, '', 7706.272727272727};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'', '', ''}, 57319, {7319, 6647829648751357319}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'v', 'w', 'x'}, 19807, {9807, 3809756166998319807}};
static short[5] g3 = {23157, 57, 3157};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
