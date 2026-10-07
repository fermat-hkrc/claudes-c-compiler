#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {22686, '}', 2062.3636363636365};
static float g1 = 19080;
static unsigned char[16] g2 = { 183, 56, 185, 58, 187, 60, 189, 62, 191, 64, 193, 66, 195, 68, 197, 70 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g3 = {5, 5, 1506469};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
