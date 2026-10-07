#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](6));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-37));
printf("mid4=%d\\n", mid4(-50));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
