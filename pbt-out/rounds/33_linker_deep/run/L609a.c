#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-26));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(7));
printf("tent4=%d\\n", init_tent4());
printf("tent5=%d\\n", init_tent5());

    return 0;
}
