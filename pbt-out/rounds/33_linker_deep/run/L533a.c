#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-9));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-16));

    return 0;
}
