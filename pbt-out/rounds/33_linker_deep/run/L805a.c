#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(18));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("g4=%d\\n", g4);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
