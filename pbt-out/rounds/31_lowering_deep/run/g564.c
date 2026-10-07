#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 251, 88, 181, 18, 111, 204, 41, 134, 227, 64, 157, 250, 87, 180, 17, 110 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'x', 'y', 'z'}, 46997, {6997, -4020168672457304619}};
static unsigned char[16] g2 = { 38, 48, 58, 68, 78, 88, 98, 108, 118, 128, 138, 148, 158, 168, 178, 188 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'o', 'p', 'q'}, 96778, {16778, 5187044165716396778}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
