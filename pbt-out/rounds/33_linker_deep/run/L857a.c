#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(1));
printf("tbl2=%d\\n", tbl2[1](-12));
printf("tbl3=%d\\n", tbl3[1](-12));
printf("tbl4=%d\\n", tbl4[1](-5));
printf("mid5=%d\\n", mid5(-34));

    return 0;
}
