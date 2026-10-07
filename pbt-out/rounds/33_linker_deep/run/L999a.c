#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-29));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(11));
printf("tbl4=%d\\n", tbl4[1](13));
printf("mid5=%d\\n", mid5(-59));

    return 0;
}
