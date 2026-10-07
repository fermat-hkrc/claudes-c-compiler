#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-16));
printf("mid4=%d\\n", mid4(-46));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
