#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(3));
printf("f2=%d\\n", f2(9));
printf("tbl3=%d\\n", tbl3[1](-16));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-4));

    return 0;
}
