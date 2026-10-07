#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(4));
printf("g4=%d\\n", g4);
printf("g5=%d\\n", g5);

    return 0;
}
