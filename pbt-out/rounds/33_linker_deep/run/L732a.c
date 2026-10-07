#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-31));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-5));
printf("f4=%d\\n", f4(-11));
printf("g5=%d\\n", g5);

    return 0;
}
