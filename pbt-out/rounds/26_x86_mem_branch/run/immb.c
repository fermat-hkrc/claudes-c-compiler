// 内联汇编测试：imm 越界在 ccc 汇编后实际加载了什么值
#include <stdio.h>
int main(void) {
    long r;
    __asm__ volatile ("movq $0x100000000, %%rax" : "=a"(r) :: );
    // 0x100000000 = 4294967296 = 2^32 → 如果截断为 imm32 → rax = 0
    printf("rax = %ld (expected 4294967296)\n", r);
    return 0;
}
