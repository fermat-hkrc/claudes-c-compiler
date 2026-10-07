#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-7));
printf("f2=%d\\n", f2(-36));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-21));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
