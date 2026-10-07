#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(3));
printf("tbl2=%d\\n", tbl2[1](0));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-4));
printf("g5=%d\\n", g5);

    return 0;
}
