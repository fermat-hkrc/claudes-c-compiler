#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int arr3[4];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(0));
printf("g2=%d\\n", g2);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tbl4=%d\\n", tbl4[1](13));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
