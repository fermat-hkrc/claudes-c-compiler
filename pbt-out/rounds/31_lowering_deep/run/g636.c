#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 243, 24, 61, 98, 135, 172, 209, 246, 27, 64, 101, 138, 175, 212, 249, 30 };
static int[4][3] g1 = {{946, 1946, 946}, {3946, 3946}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {3, 3, 5928963};
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 17747739452853451151};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
