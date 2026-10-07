#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 111, 248, 129, 10, 147, 28, 165, 46, 183, 64, 201, 82, 219, 100, 237, 118 };
static struct { unsigned x:1; unsigned y:31; } g1 = {1, 5477348712304104915};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {2, 2, 7876066};
static float g3 = 11169;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
