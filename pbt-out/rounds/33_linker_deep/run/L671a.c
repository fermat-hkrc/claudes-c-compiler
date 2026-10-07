#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-42));
printf("tbl2=%d\\n", tbl2[1](-6));
printf("tbl3=%d\\n", tbl3[1](5));
printf("f4=%d\\n", f4(5));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
