#include <stdio.h>
#include <string.h>

static struct { unsigned a:3; unsigned b:5; int c:24; } g0 = {3, 19, 2955059};
static unsigned char[16] g1 = { 177, 8, 95, 182, 13, 100, 187, 18, 105, 192, 23, 110, 197, 28, 115, 202 };
static struct { unsigned x:1; unsigned y:31; } g2 = {1, 7001917301610417127};
static struct { unsigned a:3; unsigned b:5; int c:24; } g3 = {4, 12, -1357396};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
