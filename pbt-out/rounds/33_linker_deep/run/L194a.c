#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int f3(int);
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](14));
printf("g2=%d\\n", g2);
printf("f3=%d\\n", f3(-36));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-39));

    return 0;
}
