#include <stdio.h>
extern int g0; /* 至少一个 extern 保证链接器工作 */
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("tent3=%d\\n", init_tent3());
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-3));

    return 0;
}
