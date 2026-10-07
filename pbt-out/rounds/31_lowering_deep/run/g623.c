#include <stdio.h>
#include <string.h>

static long long g0 = 2935279814u;
static struct { unsigned x:1; unsigned y:31; } g1 = {0, 4973494215337153490};
static unsigned char[16] g2 = { 107, 216, 69, 178, 31, 140, 249, 102, 211, 64, 173, 26, 135, 244, 97, 206 };
static unsigned char[16] g3 = { 7, 184, 105, 26, 203, 124, 45, 222, 143, 64, 241, 162, 83, 4, 181, 102 };

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
