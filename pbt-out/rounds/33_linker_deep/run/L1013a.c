#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("f2=%d\\n", f2(-23));
printf("mid3=%d\\n", mid3(-38));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](19));

    return 0;
}
