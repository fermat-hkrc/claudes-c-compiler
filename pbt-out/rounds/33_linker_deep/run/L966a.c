#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](3));
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("f5=%d\\n", f5(13));

    return 0;
}
