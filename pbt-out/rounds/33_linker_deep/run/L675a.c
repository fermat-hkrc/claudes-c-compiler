#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-7));
printf("tbl4=%d\\n", tbl4[1](19));
printf("mid5=%d\\n", mid5(-26));

    return 0;
}
