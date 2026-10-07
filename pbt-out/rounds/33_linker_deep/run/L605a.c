#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](18));
printf("mid4=%d\\n", mid4(-46));
printf("f5=%d\\n", f5(-35));

    return 0;
}
