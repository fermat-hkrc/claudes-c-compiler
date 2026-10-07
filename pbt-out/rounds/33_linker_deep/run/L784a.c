#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(5));
printf("tbl2=%d\\n", tbl2[1](12));
printf("mid3=%d\\n", mid3(-20));
printf("f4=%d\\n", f4(6));
printf("f5=%d\\n", f5(-13));

    return 0;
}
