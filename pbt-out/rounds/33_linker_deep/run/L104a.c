#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int mid3(int);
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(12));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
