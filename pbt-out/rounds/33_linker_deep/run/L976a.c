#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](9));
printf("f2=%d\\n", f2(-22));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-16));
printf("tbl5=%d\\n", tbl5[1](-19));

    return 0;
}
