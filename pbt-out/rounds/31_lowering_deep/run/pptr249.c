#include <stdio.h>
#include <string.h>
static int a = 11;
static int arr[6] = {10,20,30,40,50,60};
static struct S { char c; int x[4]; double d; } s = {'Q', {1,2,3,4}, 9.5};
static int *p1 = &a;
static int *p2 = &arr[4];
static int *p3 = arr[5];
static int *p4 = s.x;
static void *p5 = &s.d;
static int *p6 = &arr[2] + 3;
static int *ps[3] = {&a, &arr[4], s.x + 2};
static char *msg = "beta7";
static char *tab[3] = {"alpha", "beta7", "gammaXY"};
static char *sel = tab[1];
int main(void) {
    printf("%ld %ld %ld %ld %ld %ld\n",
        (long)((char*)p1 - (char*)&a),
        (long)((char*)p2 - (char*)arr),
        (long)((char*)p3 - (char*)arr),
        (long)((char*)p4 - (char*)&s),
        (long)((char*)p5 - (char*)&s),
        (long)((char*)p6 - (char*)arr));
    for (int i = 0; i < 3; i++)
        printf("ps%d=%ld\n", i, (long)((char*)ps[i] - (char*)arr));
    printf("msg=%s\n", msg);
    for (int i = 0; i < 3; i++) printf("t%d=%s/%ld\n", i, tab[i], (long)strlen(tab[i]));
    printf("sel=%s\n", sel);
    return 0;
}