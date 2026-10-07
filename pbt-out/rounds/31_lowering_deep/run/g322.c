#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 145, 8, 127, 246, 109, 228, 91, 210, 73, 192, 55, 174, 37, 156, 19, 138 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {5, 5, -1159515};
static short[5] g2 = {3647, 47, 3647};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'', '', ''}, 94725, {24725, 172909731299994725}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
