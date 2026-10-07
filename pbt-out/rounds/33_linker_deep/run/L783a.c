#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](8));
printf("tent2=%d\\n", init_tent2());
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-44));
printf("f5=%d\\n", f5(-1));

    return 0;
}
