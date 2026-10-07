#include <stdio.h>
#include <string.h>

static char[8] g0 = {'j', 'J', '', 161};
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {7, 23, -1765609};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'o', 'p', 'q'}, 16688, {26688, 4630313523091916688}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g3 = {3, 27, -4882725};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
