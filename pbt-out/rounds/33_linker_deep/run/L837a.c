#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-38));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-3));
printf("f4=%d\\n", f4(-39));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
