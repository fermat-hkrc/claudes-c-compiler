#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("tent3=%d\\n", init_tent3());
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(13));

    return 0;
}
