#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0x4efae0ca};
static struct { int a; char b; double c; } g1 = {23351, '|', 2122.818181818182};
static unsigned char[16] g2 = { 246, 176, 106, 36, 222, 152, 82, 12, 198, 128, 58, 244, 174, 104, 34, 220 };
static struct { unsigned x:1; unsigned y:31; } g3 = {0, 12124328899407713982};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
