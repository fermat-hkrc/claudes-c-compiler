#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-6));
printf("tbl2=%d\\n", tbl2[1](15));
printf("f3=%d\\n", f3(4));
printf("g4=%d\\n", g4);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
