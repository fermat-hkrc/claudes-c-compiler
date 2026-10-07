#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("f3=%d\\n", f3(-30));
printf("f4=%d\\n", f4(-24));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
