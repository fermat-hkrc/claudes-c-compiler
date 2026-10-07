#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-2));
printf("tbl3=%d\\n", tbl3[1](18));
printf("mid4=%d\\n", mid4(-35));
printf("g5=%d\\n", g5);

    return 0;
}
