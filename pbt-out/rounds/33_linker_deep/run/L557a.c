#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(18));
printf("f5=%d\\n", f5(-11));

    return 0;
}
