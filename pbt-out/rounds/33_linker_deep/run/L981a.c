#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(0));
printf("tbl2=%d\\n", tbl2[1](0));
printf("f3=%d\\n", f3(-1));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("g5=%d\\n", g5);

    return 0;
}
