#include <stdio.h>
#include <string.h>

static struct { char x[3]; int y; struct { short p; long q; } z; } g0 = {{'p', 'q', 'r'}, 94555, {24555, 3745154309067294555}};
static short[5] g1 = {28839, 39, 28839};
static struct { unsigned x:1; unsigned y:31; } g2 = {0, 14714921212551786488};
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 9673224681893040759};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
