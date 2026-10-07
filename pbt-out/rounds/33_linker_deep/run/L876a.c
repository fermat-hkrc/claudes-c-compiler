#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(2));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-6));
printf("f4=%d\\n", f4(-25));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
