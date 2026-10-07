#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-42));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](18));
printf("g5=%d\\n", g5);

    return 0;
}
