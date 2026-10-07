#include <stdio.h>
extern int g0;
extern int g1;
extern int g2;
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-45));
printf("tbl4=%d\\n", tbl4[1](17));
printf("tbl5=%d\\n", tbl5[1](2));

    return 0;
}
