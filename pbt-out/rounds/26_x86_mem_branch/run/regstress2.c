#include <stdio.h>
volatile int sink = 0;
long f(long n) {
    long a=n+1, b=n+2, c=n+3, d=n+4, e=n+5, f=n+6, g=n+7, h=n+8;
    long i=n+9, j=n+10, k=n+11, l=n+12, m=n+13, o=n+14, p=n+15, q=n+16;
    sink++;  // volatile write forces all to spill
    return a+b+c+d+e+f+g+h+i+j+k+l+m+o+p+q;
}
int main(void) { printf("%ld %ld\n", f(0), f(100)); return 0; }
