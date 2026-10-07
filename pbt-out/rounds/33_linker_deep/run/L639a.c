#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](17));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-49));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
