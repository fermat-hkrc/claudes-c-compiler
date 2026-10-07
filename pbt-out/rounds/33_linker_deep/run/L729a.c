#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("f2=%d\\n", f2(10));
printf("tbl3=%d\\n", tbl3[1](4));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-10));

    return 0;
}
