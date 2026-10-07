#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-9));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-11));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-53));

    return 0;
}
