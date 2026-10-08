#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-16));
printf("mid3=%d\\n", mid3(-33));
printf("tbl4=%d\\n", tbl4[1](10));
printf("f5=%d\\n", f5(-1));

    return 0;
}
