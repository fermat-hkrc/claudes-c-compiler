#include <stdio.h>
#include <string.h>
struct Flags { unsigned a : 1, b : 2, c : 3, d : 2; };  // 8 bits exactly
union Cell { struct Flags f; unsigned char byte; };
struct Row { union Cell cells[4]; int checksum; };
int main(void) {
    struct Row rows[3];
    memset(rows, 0, sizeof(rows));
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 4; j++) {
            rows[i].cells[j].f.a = (i + j) & 1;
            rows[i].cells[j].f.b = (i * 2 + j) & 3;
            rows[i].cells[j].f.c = (i + j * 3) & 7;
            rows[i].cells[j].f.d = (i * j) & 3;
        }
        rows[i].checksum = 0;
        for (int j = 0; j < 4; j++) rows[i].checksum += rows[i].cells[j].byte;
    }
    long total = 0;
    for (int i = 0; i < 3; i++) {
        total = total * 31 + rows[i].checksum;
        for (int j = 0; j < 4; j++)
            total += rows[i].cells[j].f.a * 100 + rows[i].cells[j].f.c * 7;
    }
    printf("%ld %zu %zu\n", total, sizeof(struct Flags), sizeof(union Cell));
    return 0;
}
