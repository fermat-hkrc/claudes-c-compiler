#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("f2=%d\\n", f2(-37));
printf("f3=%d\\n", f3(-19));
printf("tbl4=%d\\n", tbl4[1](16));
printf("g5=%d\\n", g5);

    return 0;
}
