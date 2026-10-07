#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(14));
printf("tbl2=%d\\n", tbl2[1](-3));
printf("g3=%d\\n", g3);
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](17));

    return 0;
}
