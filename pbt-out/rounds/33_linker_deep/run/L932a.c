#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("f3=%d\\n", f3(-40));
printf("f4=%d\\n", f4(-25));
printf("f5=%d\\n", f5(-32));

    return 0;
}
