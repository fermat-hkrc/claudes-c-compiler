#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(19));
printf("tbl2=%d\\n", tbl2[1](4));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tent4=%d\\n", init_tent4());
printf("g5=%d\\n", g5);

    return 0;
}
