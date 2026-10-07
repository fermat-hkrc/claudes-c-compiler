#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](13));
printf("tbl4=%d\\n", tbl4[1](-16));
printf("mid5=%d\\n", mid5(-6));

    return 0;
}
