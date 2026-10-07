#include <stdio.h>
extern int g0;
extern int g1;
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](-5));
printf("f4=%d\\n", f4(-38));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
