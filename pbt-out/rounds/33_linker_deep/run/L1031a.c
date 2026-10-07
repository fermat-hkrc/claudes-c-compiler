#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-29));
printf("mid3=%d\\n", mid3(11));
printf("tbl4=%d\\n", tbl4[1](12));
printf("mid5=%d\\n", mid5(1));

    return 0;
}
