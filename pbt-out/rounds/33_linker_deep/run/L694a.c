#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](-19));
printf("tent3=%d\\n", init_tent3());
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](5));

    return 0;
}
