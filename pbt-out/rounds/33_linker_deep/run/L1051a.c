#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-39));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-53));
printf("g5=%d\\n", g5);

    return 0;
}
