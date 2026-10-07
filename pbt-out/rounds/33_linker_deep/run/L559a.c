#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-20));
printf("tbl2=%d\\n", tbl2[1](4));
printf("mid3=%d\\n", mid3(-2));
printf("mid4=%d\\n", mid4(9));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
