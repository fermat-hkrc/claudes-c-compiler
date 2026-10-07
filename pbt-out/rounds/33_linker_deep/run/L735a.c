#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](19));
printf("tbl2=%d\\n", tbl2[1](-12));
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("g5=%d\\n", g5);

    return 0;
}
