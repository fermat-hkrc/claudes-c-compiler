#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(15));
printf("mid2=%d\\n", mid2(-16));
printf("g3=%d\\n", g3);
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-54));

    return 0;
}
