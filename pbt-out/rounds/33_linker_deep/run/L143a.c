#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(18));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-19));
printf("mid5=%d\\n", mid5(-36));

    return 0;
}
