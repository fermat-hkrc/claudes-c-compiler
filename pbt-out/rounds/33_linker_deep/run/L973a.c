#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-43));
printf("tbl3=%d\\n", tbl3[1](16));
printf("tent4=%d\\n", init_tent4());
printf("mid5=%d\\n", mid5(10));

    return 0;
}
