#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int f3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-30));
printf("f2=%d\\n", f2(-26));
printf("f3=%d\\n", f3(-31));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-4));

    return 0;
}
