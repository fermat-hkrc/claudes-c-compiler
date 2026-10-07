#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-11));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](6));
printf("tbl5=%d\\n", tbl5[1](13));

    return 0;
}
