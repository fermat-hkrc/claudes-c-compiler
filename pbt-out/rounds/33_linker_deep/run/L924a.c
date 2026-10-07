#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-45));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](-13));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("f5=%d\\n", f5(-43));

    return 0;
}
