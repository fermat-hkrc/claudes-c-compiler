struct ID { long i; double d; };
struct ID f(struct ID v, int k) { struct ID r = { v.i * k, v.d + k }; return r; }
long g(long a, long b) { return a * b + 3; }
double h(double x) { return x * 1.5 + 2.0; }
int main(void) { struct ID v = f((struct ID){ 3, 0.5 }, 4); printf("%ld %.1f %ld %.2f\n", v.i, v.d, g(6, 7), h(1.25)); return 0; }
