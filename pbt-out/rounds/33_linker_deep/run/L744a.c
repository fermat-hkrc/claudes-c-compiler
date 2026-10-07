#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](9));
printf("tbl2=%d\\n", tbl2[1](-5));
printf("tbl3=%d\\n", tbl3[1](-6));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-37));

    return 0;
}
