#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(17));
printf("tbl2=%d\\n", tbl2[1](14));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-25));
printf("mid5=%d\\n", mid5(-14));

    return 0;
}
