#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(1));
printf("mid3=%d\\n", mid3(-14));
printf("tbl4=%d\\n", tbl4[1](18));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
