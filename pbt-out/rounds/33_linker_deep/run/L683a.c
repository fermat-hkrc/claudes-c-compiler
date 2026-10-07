#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-10));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-15));
printf("tbl4=%d\\n", tbl4[1](7));
printf("f5=%d\\n", f5(-16));

    return 0;
}
