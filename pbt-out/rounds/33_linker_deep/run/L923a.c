#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-8));
printf("f3=%d\\n", f3(-29));
printf("mid4=%d\\n", mid4(-25));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
