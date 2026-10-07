#include <stdio.h>
static int data[4] = { 1, 2, 3, 4 };
int main(void) {
    int sum = 0;
    __asm__ volatile("" ::: "memory");  // compiler barrier
    for (int i = 0; i < 4; i++) sum += data[i];
    __asm__ volatile("" ::: "memory");
    printf("%d\n", sum);
    return 0;
}
