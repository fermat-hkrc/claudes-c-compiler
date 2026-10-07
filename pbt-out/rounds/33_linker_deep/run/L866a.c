#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](17));
printf("g3=%d\\n", g3);
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-50));

    return 0;
}
