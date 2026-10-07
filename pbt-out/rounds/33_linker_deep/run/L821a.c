#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(-10));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-50));
printf("tbl5=%d\\n", tbl5[1](-7));

    return 0;
}
