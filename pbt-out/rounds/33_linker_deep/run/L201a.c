#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-22));
printf("tbl2=%d\\n", tbl2[1](1));
printf("mid3=%d\\n", mid3(-21));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(8));

    return 0;
}
