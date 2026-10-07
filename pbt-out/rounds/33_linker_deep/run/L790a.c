#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(8));
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("mid5=%d\\n", mid5(-7));

    return 0;
}
