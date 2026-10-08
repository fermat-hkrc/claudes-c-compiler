#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int arr4[4];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-10));
printf("tbl2=%d\\n", tbl2[1](13));
printf("g3=%d\\n", g3);
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
