#include <stdio.h>
#include <string.h>
int matrix[8][8];
void fill(void) {
    for (int i = 0; i < 8; i++)
        for (int j = 0; j < 8; j++)
            matrix[i][j] = i * 8 + j;
}
int search(int target) {
    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            if (matrix[i][j] == target) goto found;
        }
    }
    return -1;
found:
    return matrix[0][0] + matrix[7][7];
}
int cascade(int x) {
    int r = 0;
    switch (x) {
        case 0: r += 1;
        case 1: r += 2;     // fall through
        case 2: r += 4; break;
        case 3: r += 8;
        default: r += 16;
    }
    return r;
}
int main(void) {
    fill();
    printf("%d %d %d\n", search(0), search(63), search(-1));
    for (int i = -1; i <= 4; i++) printf("%d ", cascade(i));
    printf("\n");
    return 0;
}
