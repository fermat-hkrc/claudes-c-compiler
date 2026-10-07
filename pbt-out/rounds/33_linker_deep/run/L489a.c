#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-37));
printf("f2=%d\\n", f2(-32));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](1));
printf("tbl5=%d\\n", tbl5[1](19));

    return 0;
}
