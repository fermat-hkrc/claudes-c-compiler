#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(6));
printf("tbl2=%d\\n", tbl2[1](-17));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(1));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
