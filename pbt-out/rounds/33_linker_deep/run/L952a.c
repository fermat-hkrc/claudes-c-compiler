#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(1));
printf("mid2=%d\\n", mid2(-29));
printf("f3=%d\\n", f3(-28));
printf("mid4=%d\\n", mid4(-21));
printf("tbl5=%d\\n", tbl5[1](6));

    return 0;
}
