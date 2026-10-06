#include <stdio.h>
_Thread_local int tls_ctr = 0;
static long fib_memo[50] = { 0, 1 };
long fib(int n) {
    if (n < 2 || fib_memo[n]) return fib_memo[n];
    return fib_memo[n] = fib(n-1) + fib(n-2);
}
int classify(char c) {
    switch (c) {
        case 'a': case 'e': case 'i': case 'o': case 'u': return 1;
        case 'A': case 'E': case 'I': case 'O': case 'U': return 2;
        case '0'...'9': return 3;
        case ' ': case '\t': case '\n': return 4;
        default: return 0;
    }
}
int main(void) {
    tls_ctr = 5;
    long f = fib(45);
    int counts[5] = { 0 };
    const char *text = "Hello World 123 AEIOU aeiou\t\n";
    for (const char *p = text; *p; p++) counts[classify(*p)]++;
    printf("%ld %d %d %d %d %d %d\n",
        f, tls_ctr, counts[0], counts[1], counts[2], counts[3], counts[4]);
    return 0;
}
