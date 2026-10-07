#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](11));
printf("f3=%d\\n", f3(-25));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-15));

    return 0;
}
