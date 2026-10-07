#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(1));
printf("f2=%d\\n", f2(14));
printf("tbl3=%d\\n", tbl3[1](0));
printf("mid4=%d\\n", mid4(-32));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
