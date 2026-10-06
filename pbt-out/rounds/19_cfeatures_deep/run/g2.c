struct Mix { long i; double d; };        // INTEGER + SSE mixed eightbytes
double f(struct Mix m) { return m.i * 10 + m.d; }
struct Mix g(long i, double d) { struct Mix m = { i, d }; return m; }
int main(void) { printf("%.17g %.17g\n", f(g(3, 0.5)), f((struct Mix){ 7, 2.25 })); return 0; }
