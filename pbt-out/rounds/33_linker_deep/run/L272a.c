#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](8));
printf("f3=%d\\n", f3(-25));
printf("f4=%d\\n", f4(14));
printf("mid5=%d\\n", mid5(-36));

    return 0;
}
