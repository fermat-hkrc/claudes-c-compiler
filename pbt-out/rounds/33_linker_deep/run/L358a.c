#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](-17));
printf("mid3=%d\\n", mid3(-22));
printf("f4=%d\\n", f4(-36));
printf("f5=%d\\n", f5(3));

    return 0;
}
