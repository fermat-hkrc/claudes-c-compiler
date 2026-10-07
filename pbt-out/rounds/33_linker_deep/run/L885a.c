#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(2));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-39));
printf("f4=%d\\n", f4(14));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
