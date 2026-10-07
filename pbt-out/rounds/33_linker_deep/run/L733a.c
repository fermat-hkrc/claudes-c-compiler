#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-9));
printf("tbl2=%d\\n", tbl2[1](9));
printf("mid3=%d\\n", mid3(-47));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
