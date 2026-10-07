#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-4));
printf("tbl2=%d\\n", tbl2[1](18));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(1));
printf("tbl5=%d\\n", tbl5[1](-2));

    return 0;
}
