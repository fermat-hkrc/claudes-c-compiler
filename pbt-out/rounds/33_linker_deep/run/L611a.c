#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(9));
printf("tbl2=%d\\n", tbl2[1](-1));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(12));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
