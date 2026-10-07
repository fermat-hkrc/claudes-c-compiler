#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("f3=%d\\n", f3(-2));
printf("tbl4=%d\\n", tbl4[1](6));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
