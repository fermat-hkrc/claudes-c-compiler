#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-5));
printf("tbl2=%d\\n", tbl2[1](6));
printf("mid3=%d\\n", mid3(-31));
printf("tent4=%d\\n", init_tent4());
printf("tent5=%d\\n", init_tent5());

    return 0;
}
