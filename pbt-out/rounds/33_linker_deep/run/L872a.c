#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](-13));
printf("tbl3=%d\\n", tbl3[1](18));
printf("tbl4=%d\\n", tbl4[1](-5));
printf("g5=%d\\n", g5);

    return 0;
}
