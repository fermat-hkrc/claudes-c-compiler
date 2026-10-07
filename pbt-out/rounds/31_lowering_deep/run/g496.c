#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 117, 40, 219, 142, 65, 244, 167, 90, 13, 192, 115, 38, 217, 140, 63, 242 };
static long long g1 = 1621751588u;
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {1, 9, 5854217};
static struct { unsigned x:1; unsigned y:31; } g3 = {0, 12344579472885500396};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
