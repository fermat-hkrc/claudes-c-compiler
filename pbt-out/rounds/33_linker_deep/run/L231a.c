#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-18));
printf("mid2=%d\\n", mid2(-5));
printf("mid3=%d\\n", mid3(-44));
printf("mid4=%d\\n", mid4(18));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
