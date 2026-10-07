#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 161, 136, 111, 86, 61, 36, 11, 242, 217, 192, 167, 142, 117, 92, 67, 42 };
static union { int i; char c[8]; double d; } g1 = {0x14550284};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'u', 'v', 'w'}, 51230, {21230, 1933381792711851230}};
static int g3 = 705003;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
