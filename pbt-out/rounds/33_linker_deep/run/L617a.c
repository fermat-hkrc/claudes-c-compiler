#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](7));
printf("f2=%d\\n", f2(-4));
printf("tbl3=%d\\n", tbl3[1](18));
printf("f4=%d\\n", f4(-27));
printf("mid5=%d\\n", mid5(-24));

    return 0;
}
