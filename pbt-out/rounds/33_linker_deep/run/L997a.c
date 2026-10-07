#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(18));
printf("mid2=%d\\n", mid2(-15));
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-1));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
