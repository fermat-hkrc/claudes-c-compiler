#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-12));
printf("mid2=%d\\n", mid2(-40));
printf("mid3=%d\\n", mid3(-32));
printf("tent4=%d\\n", init_tent4());
printf("tent5=%d\\n", init_tent5());

    return 0;
}
