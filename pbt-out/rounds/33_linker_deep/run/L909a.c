#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-11));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](7));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
