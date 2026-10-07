#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-14));
printf("mid2=%d\\n", mid2(-2));
printf("mid3=%d\\n", mid3(-25));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-12));

    return 0;
}
