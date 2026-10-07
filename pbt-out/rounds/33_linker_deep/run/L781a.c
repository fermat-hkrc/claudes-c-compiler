#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-3));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
