#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](0));
printf("mid4=%d\\n", mid4(-26));
printf("tbl5=%d\\n", tbl5[1](11));

    return 0;
}
