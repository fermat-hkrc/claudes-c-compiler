#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-41));
printf("mid2=%d\\n", mid2(-29));
printf("f3=%d\\n", f3(5));
printf("tbl4=%d\\n", tbl4[1](6));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
