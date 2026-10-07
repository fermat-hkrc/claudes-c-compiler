#include <stdio.h>
extern int g0;
extern int g1;
extern int arr2[4];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-5));
printf("mid5=%d\\n", mid5(-59));

    return 0;
}
