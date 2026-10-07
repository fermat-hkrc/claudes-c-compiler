#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-24));
printf("mid2=%d\\n", mid2(13));
printf("f3=%d\\n", f3(-26));
printf("tbl4=%d\\n", tbl4[1](-1));
printf("mid5=%d\\n", mid5(-15));

    return 0;
}
