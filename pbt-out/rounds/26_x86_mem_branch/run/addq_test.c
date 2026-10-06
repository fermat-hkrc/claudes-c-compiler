#include <stdio.h>
int main(void) {
    unsigned long long r = 5;
    // addq with a value that fits in imm32 → works fine
    __asm__ volatile ("addq $100, %0" : "+r"(r));
    printf("add $100: %llu\n", r);
    return 0;
}
