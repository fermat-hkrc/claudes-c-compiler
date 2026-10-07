#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-28));
printf("mid2=%d\\n", mid2(-43));
printf("f3=%d\\n", f3(-13));
printf("mid4=%d\\n", mid4(2));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
