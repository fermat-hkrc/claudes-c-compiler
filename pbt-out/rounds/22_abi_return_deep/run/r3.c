struct ID { long i; double d; };
struct ID bump(struct ID v) { struct ID r = { v.i + 1, v.d * 2 }; return r; }
int main(void) { struct ID v = bump((struct ID){ 3, 1.25 }), w = bump(v); printf("%ld %.17g %ld\n", v.i, v.d, w.i); return 0; }
