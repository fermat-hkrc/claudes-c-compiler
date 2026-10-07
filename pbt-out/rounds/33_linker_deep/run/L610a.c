#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(14));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(10));
printf("mid4=%d\\n", mid4(-27));
printf("tbl5=%d\\n", tbl5[1](7));

    return 0;
}
