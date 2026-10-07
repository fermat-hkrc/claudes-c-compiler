#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](17));
printf("tbl2=%d\\n", tbl2[1](-7));
printf("f3=%d\\n", f3(5));
printf("tbl4=%d\\n", tbl4[1](-11));
printf("mid5=%d\\n", mid5(-34));

    return 0;
}
