#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(8));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-13));

    return 0;
}
