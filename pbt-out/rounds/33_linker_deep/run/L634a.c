#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-44));
printf("f2=%d\\n", f2(-15));
printf("f3=%d\\n", f3(-33));
printf("mid4=%d\\n", mid4(16));
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
