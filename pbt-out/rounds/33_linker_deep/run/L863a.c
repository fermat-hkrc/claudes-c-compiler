#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](-8));
printf("tbl3=%d\\n", tbl3[1](9));
printf("mid4=%d\\n", mid4(-40));
printf("f5=%d\\n", f5(-33));

    return 0;
}
