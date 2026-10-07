#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](-3));
printf("mid3=%d\\n", mid3(-59));
printf("tent4=%d\\n", init_tent4());
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
