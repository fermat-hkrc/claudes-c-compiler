#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](4));
printf("mid3=%d\\n", mid3(-37));
printf("f4=%d\\n", f4(-5));
printf("f5=%d\\n", f5(-28));

    return 0;
}
