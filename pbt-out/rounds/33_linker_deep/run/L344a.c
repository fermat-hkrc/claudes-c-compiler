#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-29));
printf("mid3=%d\\n", mid3(3));
printf("tbl4=%d\\n", tbl4[1](7));
printf("f5=%d\\n", f5(-17));

    return 0;
}
