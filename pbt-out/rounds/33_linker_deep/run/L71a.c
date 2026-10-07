#include <stdio.h>
extern int g0;
extern int g1;
extern int arr2[4];
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
