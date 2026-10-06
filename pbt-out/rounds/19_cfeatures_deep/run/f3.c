long apply(long (*f)(long), long x) { return f(x); }
long inc(long x) { return x + 1; }
long (*tbl[2])(long) = { inc, 0 };
int main(void) { printf("%ld\n", apply(tbl[0], 41) + apply(inc, 1)); return 0; }
