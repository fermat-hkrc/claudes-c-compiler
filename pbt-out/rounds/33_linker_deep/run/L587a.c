#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-16));
printf("mid3=%d\\n", mid3(-36));
printf("f4=%d\\n", f4(12));
printf("f5=%d\\n", f5(5));

    return 0;
}
