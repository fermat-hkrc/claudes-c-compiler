#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xc36983505a08a399};
static unsigned char[16] g1 = { 23, 56, 89, 122, 155, 188, 221, 254, 31, 64, 97, 130, 163, 196, 229, 6 };
static long long g2 = 69563767u;
static union { long l; short s[4]; } g3 = {0x4dd114b8767f84ab};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
