#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-31));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](9));
printf("mid4=%d\\n", mid4(-54));
printf("f5=%d\\n", f5(-36));

    return 0;
}
