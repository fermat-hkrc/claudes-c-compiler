#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-43));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-11));
printf("f4=%d\\n", f4(-27));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
