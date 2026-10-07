#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-8));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-12));
printf("f4=%d\\n", f4(8));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
