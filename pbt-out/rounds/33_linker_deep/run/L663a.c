#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-41));
printf("f2=%d\\n", f2(-27));
printf("mid3=%d\\n", mid3(-30));
printf("tbl4=%d\\n", tbl4[1](-13));
printf("g5=%d\\n", g5);

    return 0;
}
