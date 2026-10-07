#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-52));
printf("tbl2=%d\\n", tbl2[1](15));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-16));
printf("f5=%d\\n", f5(-7));

    return 0;
}
