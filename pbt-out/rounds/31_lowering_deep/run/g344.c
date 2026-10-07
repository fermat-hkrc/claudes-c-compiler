#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 122, 208, 38, 124, 210, 40, 126, 212, 42, 128, 214, 44, 130, 216, 46, 132 };
static struct { unsigned x:1; unsigned y:31; } g1 = {1, 14150196629387399065};
static struct { int a; char b; double c; } g2 = {7776, '', 706.9090909090909};
static struct { int a; char b; double c; } g3 = {31535, '', 2866.818181818182};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
