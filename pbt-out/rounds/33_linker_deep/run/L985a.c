#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(18));
printf("f3=%d\\n", f3(-20));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
