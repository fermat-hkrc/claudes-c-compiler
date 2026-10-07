#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](2));
printf("mid2=%d\\n", mid2(13));
printf("tent3=%d\\n", init_tent3());
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-14));

    return 0;
}
