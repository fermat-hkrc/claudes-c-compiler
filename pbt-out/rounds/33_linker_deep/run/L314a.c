#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](9));
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-12));

    return 0;
}
