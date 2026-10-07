#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-25));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("mid4=%d\\n", mid4(3));
printf("f5=%d\\n", f5(-30));

    return 0;
}
