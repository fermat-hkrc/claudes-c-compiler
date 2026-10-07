#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-51));
printf("tbl4=%d\\n", tbl4[1](-16));
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
