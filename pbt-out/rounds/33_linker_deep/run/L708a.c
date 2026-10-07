#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-2));
printf("f4=%d\\n", f4(-6));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
