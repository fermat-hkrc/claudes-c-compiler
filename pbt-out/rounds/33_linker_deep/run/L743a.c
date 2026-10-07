#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](-3));
printf("tbl3=%d\\n", tbl3[1](-1));
printf("f4=%d\\n", f4(-41));
printf("mid5=%d\\n", mid5(-40));

    return 0;
}
