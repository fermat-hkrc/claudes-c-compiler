#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-1));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
