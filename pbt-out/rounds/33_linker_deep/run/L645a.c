#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](-4));
printf("mid3=%d\\n", mid3(7));
printf("mid4=%d\\n", mid4(-15));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
