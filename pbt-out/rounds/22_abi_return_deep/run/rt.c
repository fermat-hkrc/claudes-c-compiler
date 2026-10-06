struct ID { long i; double d; };
struct ID f(struct ID v, int k) { struct ID r = { v.i * k, v.d + k }; return r; }
int main(void) { struct ID v = f((struct ID){ 3, 0.5 }, 4); printf("%ld %.1f\n", v.i, v.d); return 0; }
