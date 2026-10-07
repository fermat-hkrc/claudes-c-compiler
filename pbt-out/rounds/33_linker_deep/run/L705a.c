#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-16));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-11));
printf("mid4=%d\\n", mid4(-45));
printf("g5=%d\\n", g5);

    return 0;
}
