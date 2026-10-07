#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-42));
printf("f2=%d\\n", f2(6));
printf("tent3=%d\\n", init_tent3());
printf("tent4=%d\\n", init_tent4());
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
