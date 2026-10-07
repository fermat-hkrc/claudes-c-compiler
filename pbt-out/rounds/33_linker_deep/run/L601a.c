#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-41));
printf("mid3=%d\\n", mid3(-11));
printf("mid4=%d\\n", mid4(13));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
