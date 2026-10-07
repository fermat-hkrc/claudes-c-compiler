#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(-36));
printf("tbl3=%d\\n", tbl3[1](-9));
printf("f4=%d\\n", f4(-30));
printf("mid5=%d\\n", mid5(11));

    return 0;
}
