#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-32));
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-18));
printf("tbl4=%d\\n", tbl4[1](1));
printf("mid5=%d\\n", mid5(-54));

    return 0;
}
