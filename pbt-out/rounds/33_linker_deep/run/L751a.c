#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(1));
printf("f2=%d\\n", f2(3));
printf("tent3=%d\\n", init_tent3());
printf("g4=%d\\n", g4);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
