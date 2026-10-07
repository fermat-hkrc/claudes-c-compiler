#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](19));
printf("tbl2=%d\\n", tbl2[1](-9));
printf("tent3=%d\\n", init_tent3());
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("f5=%d\\n", f5(-42));

    return 0;
}
