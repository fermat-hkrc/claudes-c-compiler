#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-13));
printf("mid4=%d\\n", mid4(-51));
printf("g5=%d\\n", g5);

    return 0;
}
