#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-37));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-44));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](11));

    return 0;
}
