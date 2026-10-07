#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](-3));
printf("f3=%d\\n", f3(-19));
printf("mid4=%d\\n", mid4(-41));
printf("tbl5=%d\\n", tbl5[1](12));

    return 0;
}
