#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(1));
printf("f3=%d\\n", f3(-18));
printf("f4=%d\\n", f4(-44));
printf("mid5=%d\\n", mid5(9));

    return 0;
}
