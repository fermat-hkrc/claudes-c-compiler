#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
extern int mid3(int);
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](6));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-29));
printf("mid4=%d\\n", mid4(-35));
printf("g5=%d\\n", g5);

    return 0;
}
