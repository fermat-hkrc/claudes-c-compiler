#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int arr2[4];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-8));
printf("tbl4=%d\\n", tbl4[1](6));
printf("mid5=%d\\n", mid5(6));

    return 0;
}
