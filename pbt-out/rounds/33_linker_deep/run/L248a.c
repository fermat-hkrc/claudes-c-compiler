#include <stdio.h>
extern int g0;
extern int f1(int);
extern int arr2[4];
extern int arr3[4];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(12));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("mid4=%d\\n", mid4(19));
printf("tbl5=%d\\n", tbl5[1](-14));

    return 0;
}
