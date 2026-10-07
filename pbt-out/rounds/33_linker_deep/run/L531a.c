#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-4));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
