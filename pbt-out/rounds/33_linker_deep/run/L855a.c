#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-8));
printf("f2=%d\\n", f2(11));
printf("tbl3=%d\\n", tbl3[1](19));
printf("f4=%d\\n", f4(-2));
printf("mid5=%d\\n", mid5(11));

    return 0;
}
