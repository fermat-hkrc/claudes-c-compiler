#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](0));
printf("tbl2=%d\\n", tbl2[1](0));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("f4=%d\\n", f4(-7));
printf("mid5=%d\\n", mid5(-35));

    return 0;
}
