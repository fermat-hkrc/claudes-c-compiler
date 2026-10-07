#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(-42));
printf("mid3=%d\\n", mid3(-6));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-3));

    return 0;
}
