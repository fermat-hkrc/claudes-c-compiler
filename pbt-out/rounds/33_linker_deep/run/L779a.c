#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-16));
printf("f2=%d\\n", f2(-30));
printf("f3=%d\\n", f3(-1));
printf("tbl4=%d\\n", tbl4[1](14));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
