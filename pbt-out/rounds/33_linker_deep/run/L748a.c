#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-12));
printf("mid2=%d\\n", mid2(-8));
printf("f3=%d\\n", f3(-30));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-14));

    return 0;
}
