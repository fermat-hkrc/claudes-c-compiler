#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-4));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-42));
printf("tbl4=%d\\n", tbl4[1](15));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
