#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](3));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-6));
printf("mid4=%d\\n", mid4(-12));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
