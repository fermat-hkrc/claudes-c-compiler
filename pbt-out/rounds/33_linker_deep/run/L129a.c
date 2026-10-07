#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int g2;
extern int mid3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-55));
printf("f4=%d\\n", f4(-3));
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
