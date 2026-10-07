#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(-2));
printf("mid3=%d\\n", mid3(-23));
printf("mid4=%d\\n", mid4(17));
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
