#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-21));
printf("f2=%d\\n", f2(-4));
printf("tbl3=%d\\n", tbl3[1](-6));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-15));

    return 0;
}
