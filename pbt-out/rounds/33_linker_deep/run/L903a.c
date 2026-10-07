#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-19));
printf("f2=%d\\n", f2(-26));
printf("f3=%d\\n", f3(-12));
printf("mid4=%d\\n", mid4(12));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
