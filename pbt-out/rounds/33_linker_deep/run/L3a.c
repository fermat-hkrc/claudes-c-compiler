#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-22));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](19));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
