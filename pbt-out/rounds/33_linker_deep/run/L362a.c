#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(6));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-16));
printf("tbl4=%d\\n", tbl4[1](-11));
printf("mid5=%d\\n", mid5(-25));

    return 0;
}
