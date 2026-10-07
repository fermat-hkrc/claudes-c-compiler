#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int arr3[4];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-7));
printf("mid2=%d\\n", mid2(16));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("f4=%d\\n", f4(-4));
printf("mid5=%d\\n", mid5(-37));

    return 0;
}
