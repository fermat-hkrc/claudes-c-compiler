#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int g2;
extern int mid3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-37));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
