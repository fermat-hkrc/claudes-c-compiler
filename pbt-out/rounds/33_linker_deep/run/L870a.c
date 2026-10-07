#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(-27));
printf("mid3=%d\\n", mid3(15));
printf("g4=%d\\n", g4);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
