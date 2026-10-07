#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-8));
printf("mid2=%d\\n", mid2(-33));
printf("f3=%d\\n", f3(-31));
printf("f4=%d\\n", f4(-20));
printf("tbl5=%d\\n", tbl5[1](6));

    return 0;
}
