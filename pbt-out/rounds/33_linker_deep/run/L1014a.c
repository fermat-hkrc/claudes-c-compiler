#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-24));
printf("f2=%d\\n", f2(-8));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-30));
printf("mid5=%d\\n", mid5(4));

    return 0;
}
