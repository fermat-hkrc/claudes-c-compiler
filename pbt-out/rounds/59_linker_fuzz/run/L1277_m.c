#include <stdio.h>
extern int sym0;
extern int sym1;
extern int sym2;
int main(void){
    printf("%d ", sym0);
    printf("%d ", sym1);
    printf("%d ", sym2);
    putchar('\n'); return 0;
}
