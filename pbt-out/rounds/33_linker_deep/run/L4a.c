#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int f2(int);
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(10));
printf("mid3=%d\\n", mid3(-38));
printf("mid4=%d\\n", mid4(11));
printf("tbl5=%d\\n", tbl5[1](-18));

    return 0;
}
