#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(9));
printf("tbl2=%d\\n", tbl2[1](3));
printf("mid3=%d\\n", mid3(-1));
printf("f4=%d\\n", f4(-27));
printf("mid5=%d\\n", mid5(14));

    return 0;
}
