#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(15));
printf("f3=%d\\n", f3(-23));
printf("f4=%d\\n", f4(-9));
printf("g5=%d\\n", g5);

    return 0;
}
