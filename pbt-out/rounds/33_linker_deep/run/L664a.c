#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-25));
printf("f2=%d\\n", f2(-42));
printf("tbl3=%d\\n", tbl3[1](17));
printf("f4=%d\\n", f4(14));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
