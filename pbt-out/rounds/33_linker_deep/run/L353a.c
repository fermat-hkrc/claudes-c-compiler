#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int g3;
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](10));
printf("f2=%d\\n", f2(14));
printf("g3=%d\\n", g3);
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-49));

    return 0;
}
