#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-21));
printf("f3=%d\\n", f3(-24));
printf("mid4=%d\\n", mid4(9));
printf("tbl5=%d\\n", tbl5[1](-5));

    return 0;
}
