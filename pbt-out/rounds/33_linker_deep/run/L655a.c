#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-17));
printf("f2=%d\\n", f2(12));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("g4=%d\\n", g4);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
