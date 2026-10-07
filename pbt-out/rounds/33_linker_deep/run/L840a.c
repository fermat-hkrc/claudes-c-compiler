#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-40));
printf("mid2=%d\\n", mid2(-28));
printf("tbl3=%d\\n", tbl3[1](-10));
printf("g4=%d\\n", g4);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
