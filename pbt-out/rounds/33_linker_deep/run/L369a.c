#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(11));
printf("tbl2=%d\\n", tbl2[1](-9));
printf("mid3=%d\\n", mid3(-14));
printf("mid4=%d\\n", mid4(-13));
printf("mid5=%d\\n", mid5(-12));

    return 0;
}
