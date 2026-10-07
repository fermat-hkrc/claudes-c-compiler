#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-16));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-3));
printf("tbl5=%d\\n", tbl5[1](-14));

    return 0;
}
