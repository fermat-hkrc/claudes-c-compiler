#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-12));
printf("tbl2=%d\\n", tbl2[1](19));
printf("mid3=%d\\n", mid3(6));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("mid5=%d\\n", mid5(-33));

    return 0;
}
