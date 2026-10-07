#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-4));
printf("mid2=%d\\n", mid2(15));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-19));
printf("mid5=%d\\n", mid5(-2));

    return 0;
}
