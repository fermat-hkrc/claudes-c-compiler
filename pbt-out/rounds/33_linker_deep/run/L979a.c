#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-41));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
