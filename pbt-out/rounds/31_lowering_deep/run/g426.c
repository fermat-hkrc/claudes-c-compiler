#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xc1dbf2395399bfcb};
static int[4][3] g1 = {{430, 430, 430}, {2430, 1430}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'', '', ''}, 53749, {13749, -3643660177934197867}};
static unsigned char[16] g3 = { 231, 184, 137, 90, 43, 252, 205, 158, 111, 64, 17, 226, 179, 132, 85, 38 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
