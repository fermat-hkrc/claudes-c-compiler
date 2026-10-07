#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](15));
printf("tbl2=%d\\n", tbl2[1](-17));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-9));
printf("g5=%d\\n", g5);

    return 0;
}
