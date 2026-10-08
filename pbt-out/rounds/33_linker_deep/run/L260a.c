#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-51));
printf("tbl2=%d\\n", tbl2[1](-11));
printf("mid3=%d\\n", mid3(-9));
printf("mid4=%d\\n", mid4(13));
printf("f5=%d\\n", f5(-16));

    return 0;
}
