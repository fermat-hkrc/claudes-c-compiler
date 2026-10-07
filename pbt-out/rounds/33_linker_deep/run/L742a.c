#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-20));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(0));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-3));

    return 0;
}
