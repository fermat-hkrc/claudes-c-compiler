#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("f3=%d\\n", f3(-42));
printf("tbl4=%d\\n", tbl4[1](-5));
printf("g5=%d\\n", g5);

    return 0;
}
