#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](1));
printf("mid3=%d\\n", mid3(6));
printf("mid4=%d\\n", mid4(-50));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
