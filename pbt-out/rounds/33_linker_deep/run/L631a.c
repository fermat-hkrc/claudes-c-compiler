#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-29));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](19));
printf("mid4=%d\\n", mid4(-51));
printf("mid5=%d\\n", mid5(-25));

    return 0;
}
