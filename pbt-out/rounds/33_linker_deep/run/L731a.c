#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-1));
printf("tbl2=%d\\n", tbl2[1](-11));
printf("f3=%d\\n", f3(-11));
printf("f4=%d\\n", f4(-14));
printf("f5=%d\\n", f5(-39));

    return 0;
}
