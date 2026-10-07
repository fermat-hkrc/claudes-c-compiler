#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](5));
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-1));
printf("f5=%d\\n", f5(8));

    return 0;
}
