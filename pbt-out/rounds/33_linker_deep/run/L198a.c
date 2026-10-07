#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-51));
printf("mid2=%d\\n", mid2(-54));
printf("tbl3=%d\\n", tbl3[1](-8));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("mid5=%d\\n", mid5(-28));

    return 0;
}
