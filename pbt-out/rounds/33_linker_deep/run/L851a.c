#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(12));
printf("mid2=%d\\n", mid2(-19));
printf("mid3=%d\\n", mid3(-20));
printf("f4=%d\\n", f4(-16));
printf("tbl5=%d\\n", tbl5[1](5));

    return 0;
}
