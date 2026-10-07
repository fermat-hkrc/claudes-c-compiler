#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-2));
printf("mid2=%d\\n", mid2(-39));
printf("mid3=%d\\n", mid3(-54));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-38));

    return 0;
}
