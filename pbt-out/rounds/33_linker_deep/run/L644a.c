#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-17));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("mid4=%d\\n", mid4(-59));
printf("tbl5=%d\\n", tbl5[1](16));

    return 0;
}
