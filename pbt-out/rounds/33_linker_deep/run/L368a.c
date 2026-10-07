#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int arr3[4];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-18));
printf("mid2=%d\\n", mid2(0));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tbl4=%d\\n", tbl4[1](-18));
printf("g5=%d\\n", g5);

    return 0;
}
