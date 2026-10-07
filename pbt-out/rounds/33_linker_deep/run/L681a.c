#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tent2=%d\\n", init_tent2());
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tent4=%d\\n", init_tent4());
printf("f5=%d\\n", f5(-7));

    return 0;
}
