#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](0));
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-14));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-26));

    return 0;
}
