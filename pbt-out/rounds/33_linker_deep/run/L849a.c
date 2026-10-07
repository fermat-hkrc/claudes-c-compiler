#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(19));
printf("tbl2=%d\\n", tbl2[1](-10));
printf("tbl3=%d\\n", tbl3[1](-14));
printf("tbl4=%d\\n", tbl4[1](6));
printf("f5=%d\\n", f5(-38));

    return 0;
}
