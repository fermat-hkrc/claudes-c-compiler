#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-11));
printf("mid3=%d\\n", mid3(4));
printf("f4=%d\\n", f4(10));
printf("g5=%d\\n", g5);

    return 0;
}
