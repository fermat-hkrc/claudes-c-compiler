#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-10));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("mid5=%d\\n", mid5(16));

    return 0;
}
