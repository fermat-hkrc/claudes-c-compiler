#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-46));
printf("f2=%d\\n", f2(-1));
printf("mid3=%d\\n", mid3(-22));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-39));

    return 0;
}
