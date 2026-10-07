#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](1));
printf("g2=%d\\n", g2);
printf("f3=%d\\n", f3(-10));
printf("tbl4=%d\\n", tbl4[1](-3));
printf("f5=%d\\n", f5(-33));

    return 0;
}
