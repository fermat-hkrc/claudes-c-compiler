#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-30));
printf("mid3=%d\\n", mid3(-56));
printf("tbl4=%d\\n", tbl4[1](-7));
printf("tbl5=%d\\n", tbl5[1](10));

    return 0;
}
