#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-12));
printf("f3=%d\\n", f3(-37));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-30));

    return 0;
}
