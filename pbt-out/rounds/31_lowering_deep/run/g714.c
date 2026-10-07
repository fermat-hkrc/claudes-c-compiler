#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 205, 232, 3, 30, 57, 84, 111, 138, 165, 192, 219, 246, 17, 44, 71, 98 };
static struct { int a; char b; double c; } g1 = {69858, 'w', 6350.727272727273};
static struct { unsigned x:1; unsigned y:31; } g2 = {1, 13898238541119114407};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'|', '}', '~'}, 94665, {14665, 4816788819695294665}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
