#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
extern int arr3[4];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-26));
printf("g2=%d\\n", g2);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("mid4=%d\\n", mid4(-37));
printf("tbl5=%d\\n", tbl5[1](0));

    return 0;
}
