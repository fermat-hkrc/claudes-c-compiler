#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-26));
printf("mid2=%d\\n", mid2(-57));
printf("f3=%d\\n", f3(-12));
printf("f4=%d\\n", f4(-20));
printf("tbl5=%d\\n", tbl5[1](16));

    return 0;
}
