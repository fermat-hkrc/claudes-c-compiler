#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(6));
printf("f4=%d\\n", f4(-16));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
