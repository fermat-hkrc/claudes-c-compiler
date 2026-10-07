#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(7));
printf("tbl3=%d\\n", tbl3[1](8));
printf("tbl4=%d\\n", tbl4[1](4));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
