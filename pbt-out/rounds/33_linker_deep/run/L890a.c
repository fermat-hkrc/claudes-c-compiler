#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](11));
printf("mid2=%d\\n", mid2(-25));
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-18));
printf("g5=%d\\n", g5);

    return 0;
}
