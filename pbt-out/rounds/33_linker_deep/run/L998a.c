#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-2));
printf("mid3=%d\\n", mid3(-57));
printf("mid4=%d\\n", mid4(-50));
printf("mid5=%d\\n", mid5(-55));

    return 0;
}
