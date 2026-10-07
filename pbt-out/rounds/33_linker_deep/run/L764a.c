#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(0));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](8));
printf("mid5=%d\\n", mid5(-15));

    return 0;
}
