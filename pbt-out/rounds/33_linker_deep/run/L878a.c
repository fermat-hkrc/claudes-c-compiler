#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-6));
printf("f2=%d\\n", f2(-33));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-58));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
