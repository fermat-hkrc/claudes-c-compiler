#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int g4;
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](0));
printf("g3=%d\\n", g3);
printf("g4=%d\\n", g4);
printf("g5=%d\\n", g5);

    return 0;
}
