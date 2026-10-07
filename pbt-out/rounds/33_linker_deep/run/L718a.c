#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-30));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-6));

    return 0;
}
