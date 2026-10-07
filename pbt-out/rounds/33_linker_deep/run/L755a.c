#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-1));
printf("mid2=%d\\n", mid2(11));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(11));
printf("f5=%d\\n", f5(10));

    return 0;
}
