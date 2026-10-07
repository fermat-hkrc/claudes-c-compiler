#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-22));
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-18));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
