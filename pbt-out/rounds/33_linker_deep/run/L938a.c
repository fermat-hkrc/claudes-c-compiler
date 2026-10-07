#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-27));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-22));
printf("tent4=%d\\n", init_tent4());
printf("mid5=%d\\n", mid5(-41));

    return 0;
}
