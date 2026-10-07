#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](4));
printf("mid2=%d\\n", mid2(-23));
printf("tent3=%d\\n", init_tent3());
printf("g4=%d\\n", g4);
printf("g5=%d\\n", g5);

    return 0;
}
