#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(7));
printf("tbl2=%d\\n", tbl2[1](-2));
printf("tbl3=%d\\n", tbl3[1](-13));
printf("f4=%d\\n", f4(-11));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
