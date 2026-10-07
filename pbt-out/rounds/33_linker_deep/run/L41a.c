#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
extern int mid3(int);
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-6));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-10));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
